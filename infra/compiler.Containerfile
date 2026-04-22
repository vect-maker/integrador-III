FROM rust:1.95-slim-bookworm

RUN apt-get update && apt-get install -y \
    mold \
    clang \
    build-essential \
    cmake \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV CARGO_TERM_COLOR=always
ENV CC=clang
ENV CXX=clang++
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold -C target-cpu=native"
