use crate::mappings;
use polars::prelude::*;

pub const COMPOSITE_KEY: [&str; 5] = [
    "department",
    "municipality",
    "supervision_area_id",
    "census_segment_id",
    "farm_id",
];

pub const COMPOSITE_KEY_CODES: [&str; 5] = ["S101", "S102", "S105", "S106", "S108"];

pub fn transform_composite_key(lf: LazyFrame) -> LazyFrame {
    let composite_key_parsing = [
        col("S105")
            .cast(DataType::UInt8)
            .alias("supervision_area_id"),
        col("S106").cast(DataType::UInt8).alias("census_segment_id"),
        col("S108").cast(DataType::UInt8).alias("farm_id"),
    ];

    let old_columns = cols(["S105", "S106", "S108"]);

    let lf = lf.with_columns(composite_key_parsing).drop(old_columns);

    let lf = transform_department(lf);
    transform_municipality(lf)
}
pub fn transform_department(lf: LazyFrame) -> LazyFrame {
    let department_keys = lit(Series::new("keys".into(), mappings::departments::KEYS));
    let department_vals = lit(Series::new("vals".into(), mappings::departments::VALS));

    let department_categories = FrozenCategories::new(mappings::departments::VALS.to_vec())
        .expect("Failed to create FrozenCategories for department");

    let department_dtype = DataType::Enum(
        department_categories.clone(),
        department_categories.mapping().clone(),
    );

    lf.with_columns([col("S101")
        .cast(DataType::UInt8)
        .replace_strict(
            department_keys,
            department_vals,
            Some(lit(Null {})),
            Some(department_dtype.clone()),
        )
        .alias("department")])
        .drop(cols(["S101"]))
}

pub fn transform_municipality(lf: LazyFrame) -> LazyFrame {
    let municipality_keys = lit(Series::new("keys".into(), mappings::municipality::KEYS));
    let municipality_vals = lit(Series::new("vals".into(), mappings::municipality::VALS));

    // extract unique values
    let mut unique_categories = mappings::departments::VALS.to_vec();
    unique_categories.sort_unstable();
    unique_categories.dedup();

    let municipality_categories = FrozenCategories::new(unique_categories)
        .expect("Failed to create FrozenCategories for municipality");

    let municipality_dtype = DataType::Enum(
        municipality_categories.clone(),
        municipality_categories.mapping().clone(),
    );

    lf.with_columns([col("S102")
        .cast(DataType::UInt16)
        .replace_strict(
            municipality_keys,
            municipality_vals,
            Some(lit(Null {})),
            Some(municipality_dtype.clone()),
        )
        .alias("municipality")])
        .drop(cols(["S102"]))
}
