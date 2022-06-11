FROM ubuntu as faiss-builder

# FAISS deps
RUN apt-get update && apt-get install wget cmake curl unzip build-essential libopenblas-dev git -y
RUN git clone https://github.com/Enet4/faiss.git

WORKDIR /faiss

# Build FAISS
RUN git checkout c_api_head
RUN cmake -B build . -DFAISS_ENABLE_C_API=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_BUILD_TYPE=Release -DFAISS_ENABLE_GPU=OFF -DFAISS_ENABLE_PYTHON=OFF
RUN make -C build -j faiss && make -C build/c_api

FROM ubuntu as builder

# FAISS and build deps
RUN apt-get update && apt-get install protobuf-compiler libopenblas-dev curl build-essential -y
# Install Cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# Update cargo registry
RUN /root/.cargo/bin/cargo install empty-library || true

WORKDIR /usr/src

# Copy .so from previous stage
COPY --from=faiss-builder /faiss/build/faiss/libfaiss.so /usr/local/lib/libfaiss.so
COPY --from=faiss-builder /faiss/build/c_api/libfaiss_c.so /usr/local/lib/libfaiss_c.so

COPY proto proto
COPY server server
RUN /root/.cargo/bin/cargo install --path server

FROM ubuntu

ENV LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib

# FAISS deps
RUN apt-get update && apt-get install -y openssl wget libopenblas-dev libgomp1 && rm -rf /var/lib/apt/lists/*
RUN GRPC_HEALTH_PROBE_VERSION=v0.3.1 && \
    wget -qO/bin/grpc_health_probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x /bin/grpc_health_probe

# Copy resources from previsou stages
COPY --from=faiss-builder /faiss/build/faiss/libfaiss.so /usr/local/lib/libfaiss.so
COPY --from=faiss-builder /faiss/build/c_api/libfaiss_c.so /usr/local/lib/libfaiss_c.so
COPY --from=builder /root/.cargo/bin/xkcd-explorer-server /usr/local/bin/xkcd-explorer-server

CMD ["xkcd-explorer-server"]
