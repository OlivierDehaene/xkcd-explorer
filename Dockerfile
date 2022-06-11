FROM ubuntu as faiss-builder

RUN apt-get update && apt-get install wget cmake curl unzip build-essential libopenblas-dev git -y
RUN git clone https://github.com/Enet4/faiss.git

WORKDIR /faiss

RUN git checkout c_api_head
RUN cmake -B build . -DFAISS_ENABLE_C_API=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_BUILD_TYPE=Release -DFAISS_ENABLE_GPU=OFF -DFAISS_ENABLE_PYTHON=OFF
RUN make -C build -j faiss && make -C build/c_api

FROM rust:1.58.1 as builder

RUN apt-get update && apt-get install protobuf-compiler -y

WORKDIR /usr/src

COPY --from=faiss-builder /faiss/build/faiss/libfaiss.so /usr/local/lib/libfaiss.so
COPY --from=faiss-builder /faiss/build/c_api/libfaiss_c.so /usr/local/lib/libfaiss_c.so

COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl wget && rm -rf /var/lib/apt/lists/*
RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x /bin/grpc_health_probe

COPY --from=faiss-builder /faiss/build/faiss/libfaiss.so /usr/local/lib/libfaiss.so
COPY --from=faiss-builder /faiss/build/c_api/libfaiss_c.so /usr/local/lib/libfaiss_c.so
COPY --from=builder /usr/local/cargo/bin/xkcd-explorer-server /usr/local/bin/xkcd-explorer-server

ENTRYPOINT ["xkcd-explorer-server"]
