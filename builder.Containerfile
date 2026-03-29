FROM rust:1.94.1-slim-bookworm 

RUN apt-get update && apt-get install -y \
    mold \
    clang \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
