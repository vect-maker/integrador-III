
FROM rust:1.94.1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y mold clang build-essential

WORKDIR /app
RUN mkdir -p /app/target

COPY . .

ENV CARGO_TERM_COLOR=always
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --target-dir /app/target && \
    cp /app/target/debug/pac /app/transformer


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/transformer /usr/local/bin/transformer


RUN useradd -ms /bin/bash appuser
USER appuser

WORKDIR /app
ENTRYPOINT ["/usr/local/bin/transformer"]
