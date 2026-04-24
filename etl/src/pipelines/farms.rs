use crate::loaders::load_data;

use crate::schema;
use crate::transformers::farms;
use anyhow::Result;
use datafusion::prelude::*;

pub async fn run_farms_pipeline(
    ctx: &SessionContext,
    source_path: &str,
    _out_dir: &str,
) -> Result<DataFrame> {
    let df = load_data(&ctx, &source_path).await?;
    let df = schema::farms::apply_farms_schema(df)?;

    // apply transfromations
    let df = farms::apply_null_imputation(df)?;
    let df = farms::apply_labor_ratios(df)?;
    let df = farms::apply_credit_logic(df)?;
    let df = farms::apply_gender_mapping(df)?;
    let df = farms::apply_operational_structure_mapping(df)?;
    let df = farms::apply_principal_activity_mapping(df)?;

    //crate::saver::save_data(df.clone(), &format!("{}/{}", out_dir, "farms_raw.parquet")).await?;

    Ok(df)
}
