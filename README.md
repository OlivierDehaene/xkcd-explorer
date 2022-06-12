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

## Run 

### Run with Tilt and Kubernetes

#### Run

```bash
make cluster-create
make up
```

#### Cleanup

```bash
make cluster-delete
```

### Run with Tilt and a local environment

#### Create Conda environment

```bash
conda create -n xkcd-explorer python=3.9
conda actiavate xkcd-explorer
```

#### Compile Faiss

As we use FAISS from Rust, we need to compile the C bindings.

```bash
brew install cmake libomp openblas

git clone git@github.com:Enet4/faiss.git
cd faiss
git checkout c_api_head

cmake -B build . -DFAISS_ENABLE_C_API=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_BUILD_TYPE=Release -DFAISS_ENABLE_GPU=OFF -DFAISS_ENABLE_PYTHON=OFF
sudo make install
sudo mv libfaiss_c.dylib /usr/local/lib
```

#### Install TorchServe dependencies

```bash
conda activate xkcd-explorer

git clone git@github.com:pytorch/serve.git
cd serve

python ./ts_scripts/install_dependencies.py 
pip install torchserve torch-model-archiver torch-workflow-archiver

cd torchserve && pip install -r requirements.txt
```

#### Install

This project requires Poetry and Cargo to run.

```bash
cd demo && poetry install
cd ../server && cargo build
```

#### Run 

```bash
conda activate xkcd-explorer
make up-local
```
