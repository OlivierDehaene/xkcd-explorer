# XKCD Explorer

A fun project where the idea is to encode the text of the XKCD comics using a machine learning model to be able to
search through all available comics with a prompt.

1. All XKCD comics were scrapped from xkcd.com and explainxkcd.com to create a 
[Hugging Face dataset](https://huggingface.co/datasets/olivierdehaene/xkcd).

2. A Hugging Face transformer model is downloaded and served using TorchServe

3. A Rust backend server is used to handle the gRPC communication with TorchServe, keep an in memory mapping of all comics 
and uses an FAISS index to search through the comic database.

4. A Gradio demo frontend is connected to the Rust backend using gRPC and can be used to search for a user given prompt
inside the database.

   
## Run with Tilt

```bash
make cluster-create
make up
```

## Cleanup

```bash
make cluster-delete
```
