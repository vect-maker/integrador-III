default:
    @just --list

enter:
  zellij --layout .ide/dev.kdl

build-builder-image:
  podman build -t "integrador-3-builder:dev" -f infra/builder.Containerfile

run-pipeline: 
  #!/usr/bin/env bash
  podman run -it --rm \
  --userns=keep-id \
  -e CARGO_TERM_COLOR=always \
  -e CARGO_HOME=/usr/local/cargo \
  -v $(pwd)/dataset-transformer:/workspace:Z \
  -v $(pwd)/data:/workspace/data:Z \
  -v cargo-target:/workspace/target \
  -v cargo-registry-cache:/usr/local/cargo/registry \
  localhost/integrador-3-builder:dev \
  cargo run
