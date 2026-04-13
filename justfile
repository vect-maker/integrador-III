default:
    @just --list

enter:
  zellij --layout .ide/dev.kdl

run-pipeline:
    #!/usr/bin/env bash
    podman build -f infra/transformer.Containerfile -t transformer-integrador-3:dev ./dataset-transformer/
    
    podman run -it --rm \
    --userns=keep-id \
    --security-opt label=disable \
    -v $(pwd)/data:/app/data:Z \
    transformer-integrador-3:dev
