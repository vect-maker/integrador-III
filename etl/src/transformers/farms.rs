use anyhow::Result;
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
