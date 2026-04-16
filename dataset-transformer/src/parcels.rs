use crate::composite_key::{COMPOSITE_KEY, transform_composite_key};
use polars::prelude::*;

pub fn transform_parcels(lf: LazyFrame) -> LazyFrame {
    let lf = transform_composite_key(lf);
    lf.group_by(COMPOSITE_KEY)
        .agg([
            col("S434")
                .count()
                .cast(DataType::Float32)
                .alias("total_parcels"),
            col("S434A")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_annual_crops"),
            col("S434B")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_permanent_crops"),
            col("S434C")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_cultivated_pasture"),
            col("S434D")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_natural_pasture"),
            col("S434E")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_forest"),
            col("S434F")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_fallow"),
            col("S434G")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_infrastructure"),
            col("S434H")
                .sum()
                .cast(DataType::Float32)
                .alias("mz_unusable"),
        ])
        .with_columns([(col("mz_annual_crops")
            + col("mz_permanent_crops")
            + col("mz_cultivated_pasture")
            + col("mz_natural_pasture")
            + col("mz_forest")
            + col("mz_fallow")
            + col("mz_infrastructure")
            + col("mz_unusable"))
        .cast(DataType::Float32)
        .alias("total_farm_manzanas")])
}
