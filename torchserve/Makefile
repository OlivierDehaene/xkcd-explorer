install: ## Install python requirements
	pip install -r requirements.txt

archive: ## Create .mar archive
	mkdir model_store || true
	torch-model-archiver --model-name TextEmbeddings \
		--version 1.0 \
		--export-path model_store \
		--force \
		--serialized-file models/sentence-transformers/multi-qa-mpnet-base-dot-v1/pytorch_model.bin \
		--handler infer.py \
		--extra-files "models/sentence-transformers/multi-qa-mpnet-base-dot-v1/config.json,models/sentence-transformers/multi-qa-mpnet-base-dot-v1/tokenizer_config.json,models/sentence-transformers/multi-qa-mpnet-base-dot-v1/vocab.txt,models/sentence-transformers/multi-qa-mpnet-base-dot-v1/tokenizer.json,models/sentence-transformers/multi-qa-mpnet-base-dot-v1/special_tokens_map.json"

start: ## Start torchserve server
	torchserve --start --model-store model_store --models te=TextEmbeddings.mar --ncs --foreground