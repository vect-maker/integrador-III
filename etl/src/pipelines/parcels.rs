use crate::loaders::load_data;

use crate::schema;
use crate::transformers::parcels;
use anyhow::Result;
use datafusion::prelude::*;

pub async fn run_parcels_pipeline(
    ctx: &SessionContext,
    source_path: &str,
    _out_dir: &str,
) -> Result<DataFrame> {
    let df = load_data(&ctx, &source_path).await?;
    let df = schema::parcels::apply_parcels_schema(df)?;

    // aply changes
    let df = parcels::apply_total_farm_manzanas(df)?;

    // crate::saver::save_data(df.clone(),&format!("{}/{}", out_dir, "parcels_raw.parquet"),).await?;

    Ok(df)
}
