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
pub fn add_composite_key_data(lf: LazyFrame) -> LazyFrame {
    let department_names = [
        "Nueva_Segovia",
        "Jinotega",
        "Madriz",
        "Esteli",
        "Chinandega",
        "Leon",
        "Matagalpa",
        "Boaco",
        "Managua",
        "Masaya",
        "Chontales",
        "Granada",
        "Carazo",
        "Rivas",
        "Rio_San_Juan",
        "RAAN",
        "RAAS",
    ];

    let department_keys = Series::new(
        "keys".into(),
        &[
            5u8, 10, 20, 25, 30, 35, 40, 50, 55, 60, 65, 70, 75, 80, 85, 91, 93,
        ],
    );

    let department_vals = Series::new("vals".into(), &department_names);

    let department_categories = FrozenCategories::new(department_names.to_vec())
        .expect("Failed to create FrozenCategories");

    let department_dtype = DataType::Enum(
        department_categories.clone(),
        department_categories.mapping().clone(),
    );

    lf.with_columns([col("department_id")
        .cast(DataType::String)
        .replace(lit(department_keys), lit(department_vals))
        .cast(department_dtype)
        .alias("department")])
}
