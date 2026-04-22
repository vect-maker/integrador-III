use crate::loaders::load_data;
use crate::saver;
use crate::schema;
use anyhow::Result;
use datafusion::prelude::*;

pub async fn run_farms_pipeline(
    ctx: &SessionContext,
    source_path: &str,
    out_dir: &str,
) -> Result<DataFrame> {
    let df = load_data(&ctx, &source_path).await?;
    let df = schema::farms::apply_farms_schema(df)?;

    saver::save_data(df.clone(), &format!("{}/{}", out_dir, "farms_raw.parquet")).await?;

    Ok(df)
}
