default:
    @just --list

enter:
  zellij --layout dev.kdl

build-builder-image:
  podman build -t "integrador-3-builder:dev" -f builder.Containerfile

run-pipeline: 
  #!/usr/bin/env bash
  podman run --rm \
  --userns=keep-id \
  -e CARGO_TERM_COLOR=always \
  -e CARGO_HOME=/usr/local/cargo \
  -v "$(pwd):/workspace:Z" \
  -v cargo-target:/workspace/target \
  -v cargo-registry-cache:/usr/local/cargo/registry \
  localhost/integrador-3-builder:dev \
  cargo run
