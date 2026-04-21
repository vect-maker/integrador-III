FROM rust:1.95-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    mold \
    clang \
    build-essential \
    cmake \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

ENV CARGO_TERM_COLOR=always
ENV CC=clang
ENV CXX=clang++
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold -C target-cpu=native"

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target-dir /app/target && \
    cp /app/target/release/etl /app/transformer
FROM debian:bookworm-slim

# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/transformer /usr/local/bin/transformer

RUN useradd -ms /bin/bash appuser
USER appuser

WORKDIR /app
ENTRYPOINT ["/usr/local/bin/transformer"]
