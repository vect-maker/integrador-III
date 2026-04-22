FROM localhost/compiler-integrador-3:dev AS builder

COPY . .

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
