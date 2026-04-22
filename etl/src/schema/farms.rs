use crate::schema::composite_key::{apply_composite_key_schema, get_domain_key_exprs};
use anyhow::{Context, Result};
use datafusion::arrow::datatypes::DataType;
use datafusion::prelude::*;

pub fn apply_farms_schema(df: DataFrame) -> Result<DataFrame> {
    // apply composite key schemas
    let df = apply_composite_key_schema(df)?;

    let mut projection = get_domain_key_exprs().to_vec();

    projection.extend(vec![
        // --- AREA & SCALE ---
        cast(col("s427"), DataType::Float32).alias("total_area_mz"),
        cast(col("s428"), DataType::Float32).alias("total_area_sqm"),
        // --- CREDIT SOURCES ---
        col("s1275a").is_not_null().alias("loan_banco"),
        col("s1275b").is_not_null().alias("loan_banco_produzcamos"),
        col("s1275c").is_not_null().alias("loan_ong"),
        col("s1275d").is_not_null().alias("loan_cooperativa"),
        col("s1275e").is_not_null().alias("loan_gobierno"),
        col("s1275f").is_not_null().alias("loan_comercial"),
        col("s1275g").is_not_null().alias("loan_prestamista"),
        col("s1275h").is_not_null().alias("loan_acopiador"),
        col("s1275i").is_not_null().alias("loan_otro"),
        // --- LOAN REQUESTS AND RECEIPTS ---
        col("s1274a1").eq(lit(1.0)).alias("req_crop"),
        col("s1274a2").eq(lit(1.0)).alias("req_livestock"),
        col("s1274a3").eq(lit(1.0)).alias("req_aquaculture"),
        col("s1274a4").eq(lit(1.0)).alias("req_forestry"),
        col("s1274b1").eq(lit(1.0)).alias("rec_crop"),
        col("s1274b2").eq(lit(1.0)).alias("rec_livestock"),
        col("s1274b3").eq(lit(1.0)).alias("rec_aquaculture"),
        col("s1274b4").eq(lit(1.0)).alias("rec_forestry"),
        // --- LABOR ---
        col("s1067").eq(lit(1.0)).alias("hired_workers"),
        cast(col("s1068a"), DataType::UInt16).alias("permanent_workers_total"),
        cast(col("s1069a"), DataType::UInt16).alias("temporal_workers_total"),
        // --- IRRIGATION ---
        col("s538").eq(lit(1.0)).alias("has_irrigation_system"),
        // --- TECHNOLOGY AND TRACTION ---
        col("s648a").eq(lit(1.0)).alias("traction_animal"),
        col("s648b").eq(lit(2.0)).alias("traction_tractor"),
        // --- PRODUCER AND ACTIVITY ---
        cast(col("s211d"), DataType::UInt8).alias("producer_gender"),
        cast(col("s322"), DataType::UInt8).alias("operational_structure"),
        cast(col("s324"), DataType::UInt8).alias("principal_activity"),
    ]);

    let df = df
        .select(projection)
        .context("Failed to apply farms schema")?;

    Ok(df)
}
