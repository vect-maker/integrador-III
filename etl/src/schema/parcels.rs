use crate::schema::composite_key::{apply_composite_key_schema, get_domain_key_exprs};
use anyhow::{Context, Result};
use datafusion::arrow::datatypes::DataType;
use datafusion::prelude::*;

pub fn apply_parcels_schema(df: DataFrame) -> Result<DataFrame> {
    // apply composite key schema
    let df = apply_composite_key_schema(df)?;

    let mut projection = get_domain_key_exprs().to_vec();

    projection.extend(vec![
        // Total parcels (S434)
        cast(col("s434"), DataType::Float32).alias("total_parcels"),
        // Land use columns (S434A - S434H)
        cast(col("s434a"), DataType::Float32).alias("mz_annual_crops"),
        cast(col("s434b"), DataType::Float32).alias("mz_permanent_crops"),
        cast(col("s434c"), DataType::Float32).alias("mz_cultivated_pasture"),
        cast(col("s434d"), DataType::Float32).alias("mz_natural_pasture"),
        cast(col("s434e"), DataType::Float32).alias("mz_fallow"),
        cast(col("s434f"), DataType::Float32).alias("mz_forest"),
        cast(col("s434g"), DataType::Float32).alias("mz_infrastructure"),
        cast(col("s434h"), DataType::Float32).alias("mz_unusable"),
    ]);
    let df = df.select(projection).context("Failed to apply schema")?;

    Ok(df)
}
