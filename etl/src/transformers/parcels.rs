use crate::dataframe::DataFrameExt;
use crate::mappings::composite_key::COMPOSITE_KEY;
use anyhow::{Context, Result};
use datafusion::arrow::datatypes::DataType;
use datafusion::functions_aggregate::expr_fn::sum;
use datafusion::prelude::*;

pub fn apply_total_farm_manzanas(df: DataFrame) -> Result<DataFrame> {
    let components = [
        "mz_annual_crops",
        "mz_permanent_crops",
        "mz_cultivated_pasture",
        "mz_natural_pasture",
        "mz_forest",
        "mz_fallow",
        "mz_infrastructure",
        "mz_unusable",
    ];

    let total_expr = components
        .iter()
        .map(|&c| coalesce(vec![col(c), lit(0.0f32)]))
        .reduce(|acc, e| acc + e)
        .context("Land use component list cannot be empty")?
        .alias("total_farm_manzanas");

    df.with_columns(vec![cast(total_expr, DataType::Float32)])
        .context("Failed to calculate total_farm_manzanas")
}

pub async fn aggregate_parcels_by_composite_key(df: DataFrame) -> Result<DataFrame> {
    let group_exprs: Vec<Expr> = COMPOSITE_KEY.iter().map(|&name| col(name)).collect();

    let aggr_exprs: Vec<Expr> = df
        .schema()
        .fields()
        .iter()
        .filter(|f| !COMPOSITE_KEY.contains(&f.name().as_str()))
        .map(|f| sum(col(f.name())).alias(f.name()))
        .collect();

    let df = df.aggregate(group_exprs, aggr_exprs)?;
    Ok(df)
}
