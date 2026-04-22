use anyhow::Result;
use datafusion::prelude::*;
use etl::env;
use etl::pipelines;
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

    // run pipelines
    let _ = pipelines::farms::run_farms_pipeline(&ctx, &app_config.farms_path, &app_config.out_dir)
        .await?;
    let _ = pipelines::parcels::run_parcels_pipeline(
        &ctx,
        &app_config.parcels_path,
        &app_config.out_dir,
    )
    .await?;

    println!("Finished processing");
    println!("Pipeline execution time: {:?}", timer.elapsed());

    Ok(())
}
