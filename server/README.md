# XKCD Explorer Server

Handles communication with TorchServe, keep an in memory mapping of all comics and uses an FAISS index to search
through the comic database.

## Install

### FAISS 

You first need to compile FAISS.

```bash
brew install cmake libomp openblas

git clone git@github.com:Enet4/faiss.git
cd faiss
git checkout c_api_head

cmake -B build . -DFAISS_ENABLE_C_API=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_BUILD_TYPE=Release -DFAISS_ENABLE_GPU=OFF -DFAISS_ENABLE_PYTHON=OFF
sudo make install
sudo mv libfaiss_c.dylib /usr/local/lib
```

```bash
# If you don't have rust installed
make rustup
make build
```

## Run

```bash
make run
```