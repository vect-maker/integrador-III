use crate::dataframe::DataFrameExt;
use crate::mappings::composite_key::{COMPOSITE_KEY, COMPOSITE_KEY_CODES};
use crate::mappings::{departments, municipality};
use anyhow::{Context, Result};
use datafusion::arrow::datatypes::DataType;
use datafusion::common::ScalarValue;
use datafusion::prelude::*;
use std::sync::OnceLock;

pub fn get_source_key_exprs() -> &'static [Expr] {
    static SOURCE_INSTANCE: OnceLock<Vec<Expr>> = OnceLock::new();
    SOURCE_INSTANCE.get_or_init(|| COMPOSITE_KEY_CODES.iter().map(|&k| col(k)).collect())
}

pub fn get_domain_key_exprs() -> &'static [Expr] {
    static DOMAIN_INSTANCE: OnceLock<Vec<Expr>> = OnceLock::new();
    DOMAIN_INSTANCE.get_or_init(|| COMPOSITE_KEY.iter().map(|&k| col(k)).collect())
}
pub fn apply_composite_key_schema(df: DataFrame) -> Result<DataFrame> {
    let dept_map = build_department_expr();
    let muni_map = build_municipality_expr();

    let df = df
        .with_columns(vec![
            dept_map.alias("department"),
            muni_map.alias("municipality"),
            cast(col("s105"), DataType::UInt8).alias("supervision_area_id"),
            cast(col("s106"), DataType::UInt16).alias("census_segment_id"),
            cast(col("s108"), DataType::UInt16).alias("farm_id"),
        ])
        .context("Failed to project composite key columns")?;

    let df = df.drop_columns(&["s101", "s102", "s105", "s106", "s108"])?;

    Ok(df)
}

pub fn build_municipality_expr() -> Expr {
    let initial_expr = when(
        col("s102").eq(lit(municipality::KEYS[0])),
        lit(municipality::VALS[0]),
    );

    municipality::KEYS
        .iter()
        .zip(municipality::VALS.iter())
        .skip(1)
        .fold(initial_expr, |mut acc, (&key, &val)| {
            acc.when(col("s102").eq(lit(key)), lit(val))
        })
        .otherwise(lit(ScalarValue::Null))
        .expect("Failed to build municipality CASE expression")
}

pub fn build_department_expr() -> Expr {
    let initial_expr = when(
        col("s101").eq(lit(departments::KEYS[0])),
        lit(departments::VALS[0]),
    );

    departments::KEYS
        .iter()
        .zip(departments::VALS.iter())
        .skip(1)
        .fold(initial_expr, |mut acc, (&key, &val)| {
            acc.when(col("s101").eq(lit(key)), lit(val))
        })
        .otherwise(lit(ScalarValue::Null))
        .expect("Failed to build department CASE expression")
}
