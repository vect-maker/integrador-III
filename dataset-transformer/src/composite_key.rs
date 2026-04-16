use polars::prelude::*;

pub const COMPOSITE_KEY: [&str; 5] = [
    "department_id",
    "municipality_id",
    "census_segment_id",
    "farm_id",
    "legal_status_id",
];
pub fn transform_composite_key(lf: LazyFrame) -> LazyFrame {
    let composite_key_parsing = [
        col("S101").cast(DataType::UInt8).alias("department_id"),
        col("S102").cast(DataType::UInt16).alias("municipality_id"),
        col("S105")
            .cast(DataType::UInt32)
            .alias("census_segment_id"),
        col("S106").cast(DataType::UInt16).alias("farm_id"),
        col("S108").cast(DataType::UInt8).alias("legal_status_id"),
    ];
    let old_columns = cols(["S101", "S102", "S105", "S106", "S108"]);

    lf.with_columns(composite_key_parsing).drop(old_columns)
}
