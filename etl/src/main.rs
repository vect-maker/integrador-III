use anyhow::Result;
use etl::env;
use etl::loaders::{load_farms_data, load_parcels_data};

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    // get env varibles
    let app_config = env::load_config()?;
    let ctx = SessionContext::new();

    // load parquet files
    let farms_df = load_farms_data(ctx, &app_config.farms_path).await?;
    let parcels_df = load_parcels_data(ctx, &app_config.parcels_path).await?;

    println!("Hello, world!");

    Ok(())
}
