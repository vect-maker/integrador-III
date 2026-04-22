use anyhow::Result;
use datafusion::prelude::*;
use etl::env;
use etl::loaders::{load_farms_data, load_parcels_data};
use etl::saver;
use etl::schema;
use std::time::Instant;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting processing of data");
    let timer = Instant::now();

    // get env varibles
    let app_config = env::load_config()?;
    let ctx = SessionContext::new();

    // load parquet files
    let farms_df = load_farms_data(&ctx, &app_config.farms_path).await?;
    let parcels_df = load_parcels_data(&ctx, &app_config.parcels_path).await?;

    // apply schema
    let farms_df = schema::farms::apply_farms_schema(farms_df)?;
    let parcels_df = schema::parcels::apply_parcels_schema(parcels_df)?;

    // save daa
    saver::save_data(
        farms_df,
        &format!("{}/{}", app_config.out_dir, "farms_raw.parquet"),
    )
    .await?;
    saver::save_data(
        parcels_df,
        &format!("{}/{}", app_config.out_dir, "parcels_raw.parquet"),
    )
    .await?;

    println!("Finished processing");
    println!("Pipeline execution time: {:?}", timer.elapsed());

    Ok(())
}
