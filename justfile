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
      evidence-dev-image npm run sources -- --changed

run-bi:
    -podman rm -f evidence-dev
    podman run -it --rm \
    --name evidence-dev \
    -p 3000:3000 \
    -v ./analytics:/app:Z \
    evidence-dev-image /bin/sh -c "npm run sources -- --watch & npm run dev -- --host 0.0.0.0"

build-bi:
  podman run -it --rm \
  -v ./analytics:/app:Z \
  -w /app \
  evidence-dev-image npm run build

build-pipeline:
  podman build -f infra/transformer.Containerfile -t transformer-integrador-3:dev ./etl

run-pipeline: 
    podman run -it --rm \
    --userns=keep-id \
    --security-opt label=disable \
    -v $(pwd)/data:/app/data:Z \
    -e FARMS_PATH=data/cenagro-2011-explotaciones-agropecuarias.parquet \
    -e PARCELS_PATH=data/cenagro-2011-parcelas-aprovechamiento-tierra.parquet \
    -e OUT_DIR=data \
    transformer-integrador-3:dev

run-legacy-pipeline:
    #!/usr/bin/env bash
    set -euo pipefail

    podman build -f infra/transformer.Containerfile -t transformer-integrador-3:dev ./dataset-transformer/
    
    podman run -it --rm \
    --userns=keep-id \
    --security-opt label=disable \
    -v $(pwd)/data:/app/data:Z \
    -e FARMS_PATH=data/cenagro-2011-explotaciones-agropecuarias.parquet \
    -e PARCELS_PATH=data/cenagro-2011-parcelas-aprovechamiento-tierra.parquet \
    -e OUT_DIR=data \
    transformer-integrador-3:dev

build-werehouse:
  duckdb analytics/sources/warehouse/warehouse.duckdb -init sql/build-warehouse.sql -c ".exit"
