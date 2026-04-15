default:
    @just --list

enter:
  zellij --layout .ide/dev.kdl

build-bi-container:
  podman build -f infra/evidence.Containerfile -t evidence-dev-image .

add-meta-bi:
    podman run -it --rm \
      -v ./analytics:/app:Z \
      -w /app \
      evidence-dev-image npm run sources

run-bi:
    -podman rm -f evidence-dev
    podman run -it --rm \
    --name evidence-dev \
    -p 3000:3000 \
    -v ./analytics:/app:Z \
    evidence-dev-image

build-bi:
  podman run -it --rm \
  -v ./analytics:/app:Z \
  -w /app \
  evidence-dev-image npm run build

run-pipeline:
    #!/usr/bin/env bash
    podman build -f infra/transformer.Containerfile -t transformer-integrador-3:dev ./dataset-transformer/
    
    podman run -it --rm \
    --userns=keep-id \
    --security-opt label=disable \
    -v $(pwd)/data:/app/data:Z \
    transformer-integrador-3:dev

build-werehouse:
  duckdb analytics/sources/warehouse/warehouse.duckdb -init sql/build-warehouse.sql -c ".exit"
