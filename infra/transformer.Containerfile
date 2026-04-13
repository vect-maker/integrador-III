FROM rust:1.94.1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y mold clang build-essential

RUN mkdir -p /src /app/target

WORKDIR /app
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"
ENV CARGO_TERM_COLOR=always

RUN --mount=type=bind,source=.,target=/src,ro,Z \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    bash -c "cd /src && cargo build --target-dir /app/target && cp /app/target/debug/pac /usr/local/bin/transformer"

ENTRYPOINT ["/usr/local/bin/transformer"]
