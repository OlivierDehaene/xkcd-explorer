# Inspired by: https://github.com/pytorch/serve/blob/master/examples/Huggingface_Transformers/Transformer_handler_generalized.py

import logging
import torch
import transformers

from abc import ABC
from pathlib import Path
from transformers import (
    AutoTokenizer,
    AutoModel,
)

from ts.torch_handler.base_handler import BaseHandler

logger = logging.getLogger(__name__)
logger.info("Transformers version %s", transformers.__version__)


class TransformersEmbeddingHandler(BaseHandler, ABC):
    """
    Transformers handler class for text embedding
    """

    def __init__(self):
        super(TransformersEmbeddingHandler, self).__init__()
        self.initialized = False

    def initialize(self, ctx):
        """In this initialize function, the embedding model is loaded.
        Args:
            ctx (context): It is a JSON Object containing information pertaining to the model artefacts parameters.
        """
        self.manifest = ctx.manifest
        properties = ctx.system_properties

        # Get model dir path from ctx
        model_dir = Path(properties.get("model_dir"))

        # Set device
        self.device = torch.device(
            "cuda:" + str(properties.get("gpu_id"))
            if torch.cuda.is_available() and properties.get("gpu_id") is not None
            else "cpu"
        )

        # Load model and send it to `device`
        self.model = AutoModel.from_pretrained(model_dir)
        self.model.to(self.device)
        self.model.eval()

        # Load tokenizer
        self.tokenizer = AutoTokenizer.from_pretrained(model_dir)

        self.initialized = True

    def preprocess(self, requests):
        """
        Basic text preprocessing using the loaded tokenizer

        Args:
            requests (str): The Input data in the form of text is passed on to the preprocess
            function.
        Returns:
            list : The preprocess function returns a list of Tensor for the size of the word tokens.
        """
        input_ids_batch = None
        attention_mask_batch = None

        for idx, data in enumerate(requests):
            # Load text data from requests
            input_text = data.get("data")
            if input_text is None:
                input_text = data.get("body")

            # Convert text if needed
            if isinstance(input_text, (bytes, bytearray)):
                input_text = input_text.decode("utf-8")

            logger.info("Received text: '%s'", input_text)

            encoded_inputs = self.tokenizer(
                input_text, padding=True, truncation=True, return_tensors="pt"
            )

            input_ids = encoded_inputs["input_ids"].to(self.device)
            attention_mask = encoded_inputs["attention_mask"].to(self.device)

            # making a batch out of the recieved requests
            # attention masks are passed for cases where input tokens are padded.
            if input_ids.shape is not None:
                if input_ids_batch is None:
                    input_ids_batch = input_ids
                    attention_mask_batch = attention_mask
                else:
                    input_ids_batch = torch.cat((input_ids_batch, input_ids), 0)
                    attention_mask_batch = torch.cat(
                        (attention_mask_batch, attention_mask), 0
                    )

        return (input_ids_batch, attention_mask_batch)

    def inference(self, input_batch):
        """
        Get the embeddings

        Args:
            input_batch (list): List of Text Tensors from the pre-process function is passed here
        Returns:
            list : It returns a list of the embeddings
        """

        input_ids_batch, attention_mask_batch = input_batch

        model_outputs = self.model(input_ids_batch, attention_mask_batch)
        embeddings = torch.nn.functional.normalize(model_outputs.last_hidden_state[:, 0])

        return embeddings
