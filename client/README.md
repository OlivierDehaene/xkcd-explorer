# XKCD Explorer Python Client

A python gRPC client library for the xkcd-explorer endpoint.

## Local Install

```shell
make install
```

### M1 Macs

```shell
conda create -n xkcd-explorer python=3.9
conda activate xkcd-explorer

make install-m1
```

## Usage

```python
from xkcd_explorer import Client, Comic

## create a client by passing "{host}:{port}"
client = Client("localhost:50051")

## call RPCs: Insert
client.insert(Comic(
    uuid=1,
    title="Barrel - Part 1",
    url="https://www.xkcd.com/1",
    image_url="https://imgs.xkcd.com/comics/barrel_cropped_(1).jpg",
    explained_url="https://www.explainxkcd.com/wiki/index.php/1:_Barrel_-_Part_1"
))

## call RPCs: Search
res = client.search(prompt="Barrel")
print(res)
```