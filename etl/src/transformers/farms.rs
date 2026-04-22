use crate::dataframe::DataFrameExt;
use crate::helpers::any_horizontal;
use crate::mappings;
use anyhow::{Context, Result};
use datafusion::arrow::datatypes::DataType;
use datafusion::prelude::*;

pub fn apply_labor_ratios(df: DataFrame) -> Result<DataFrame> {
    let total_workers = col("permanent_workers_total") + col("temporal_workers_total");

    let df = df.with_column(
        "permanent_labor_ratio",
        when(
            total_workers.clone().gt(lit(0u16)),
            cast(col("permanent_workers_total"), DataType::Float32)
                / cast(total_workers.clone(), DataType::Float32),
        )
        .otherwise(lit(0.0f32))?,
    )?;

    Ok(df)
}

pub fn apply_credit_logic(df: DataFrame) -> Result<DataFrame> {
    let loan_sources = [
        "loan_banco",
        "loan_banco_produzcamos",
        "loan_ong",
        "loan_cooperativa",
        "loan_gobierno",
        "loan_comercial",
        "loan_prestamista",
        "loan_acopiador",
        "loan_otro",
    ];

    let req_sources = [
        "req_crop",
        "req_livestock",
        "req_aquaculture",
        "req_forestry",
    ];
    let rec_sources = [
        "rec_crop",
        "rec_livestock",
        "rec_aquaculture",
        "rec_forestry",
    ];

    let has_any_loan = any_horizontal(&loan_sources)?;
    let requested_loan = any_horizontal(&req_sources)?;
    let received_loan = any_horizontal(&rec_sources)?;

    let df = df
        .with_columns(vec![
            has_any_loan.alias("has_any_loan"),
            requested_loan.alias("requested_loan"),
            received_loan.alias("received_loan"),
        ])
        .context("Failed to apply credit logic")?;

    Ok(df)
}

pub fn apply_gender_mapping(df: DataFrame) -> Result<DataFrame> {
    let keys: [u8; 2] = mappings::gender::KEYS;
    let vals: [&str; 2] = mappings::gender::VALS;

    let expr = when(col("producer_gender").eq(lit(keys[0])), lit(vals[0]))
        .when(col("producer_gender").eq(lit(keys[1])), lit(vals[1]))
        .otherwise(lit("ignorado"))?;

    let df = df
        .with_column("producer_gender", expr)
        .context("Failed to replace producer_gender with categorical labels")?;

    Ok(df)
}

pub fn apply_operational_structure_mapping(df: DataFrame) -> Result<DataFrame> {
    let keys = mappings::operational_structure::KEYS;
    let vals = mappings::operational_structure::VALS;

    let mut expr = when(col("operational_structure").eq(lit(keys[0])), lit(vals[0]));
    for i in 1..keys.len() {
        expr = expr.when(col("operational_structure").eq(lit(keys[i])), lit(vals[i]));
    }

    let df = df
        .with_column("operational_structure", expr.otherwise(lit("ignorado"))?)
        .context("Failed to replace operational_structure with categorical labels")?;

    Ok(df)
}

pub fn apply_principal_activity_mapping(df: DataFrame) -> Result<DataFrame> {
    let keys: [u8; 16] = mappings::principal_activity::KEYS;
    let vals: [&str; 16] = mappings::principal_activity::VALS;

    let mut expr = when(col("principal_activity").eq(lit(keys[0])), lit(vals[0]));
    for i in 1..keys.len() {
        expr = expr.when(col("principal_activity").eq(lit(keys[i])), lit(vals[i]));
    }

    let df = df
        .with_column("principal_activity", expr.otherwise(lit("ignorado"))?)
        .context("Failed to replace principal_activity with categorical labels")?;

    Ok(df)
}
