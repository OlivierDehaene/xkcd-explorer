# XKCD TorchServe

Download and preapre the `sentence-transformers/multi-qa-mpnet-base-dot-v1` Hugging Face model and serve it for 
embedding extraction using TorchServe.

The inference code is inspired by the torchserve tutorial 
[here](https://github.com/pytorch/serve/blob/master/examples/Huggingface_Transformers/Transformer_handler_generalized.py).

## Install

You first need to install TorchServe using the following 
[tutorial](https://github.com/pytorch/serve/blob/master/README.md#-quick-start-with-torchserve).

```bash
make install
```

## Prepare .mar torchserve archive

```bash
make archive
```

## Launch TorchServe server

```bash
make start
```