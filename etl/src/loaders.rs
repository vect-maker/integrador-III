use anyhow::{Context, Result};
use datafusion::prelude::*;

pub async fn load_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = ctx
        .read_parquet(path_str, ParquetReadOptions::default())
        .await
        .context("Failed to scan parquet")?;
    Ok(df)
}

pub async fn load_farms_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = load_data(ctx, file)
        .await
        .context("Failed to load farms data")?;

    Ok(df)
}

pub async fn load_parcels_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = load_data(ctx, file)
        .await
        .context("Failed to load farms data")?;

    Ok(df)
}
