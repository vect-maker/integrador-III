use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

const FARMS_METADATA: &str = "cenagro-2011-explotaciones-agropecuarias-metadata.json";
const PARCELS_METADATA: &str = "cenagro-2011-parcelas-aprovechamiento-tierra-metadata.json";

#[derive(Deserialize, Debug)]
struct SPSSMetadata {
    variable_value_labels: serde_json::Value,
    column_names_to_labels: serde_json::Value,
}

fn load_data() -> (LazyFrame, LazyFrame) {
    let explotaciones_agropecuarias_file = "data/cenagro-2011-explotaciones-agropecuarias.parquet";
    let parcelas_aprovechamiento_tierra_file =
        "data/cenagro-2011-parcelas-aprovechamiento-tierra.parquet";

    let lf_farms: LazyFrame =
        LazyFrame::scan_parquet(explotaciones_agropecuarias_file.into(), Default::default())
            .expect("Could not load farms data");
    let lf_parcels: LazyFrame = LazyFrame::scan_parquet(
        parcelas_aprovechamiento_tierra_file.into(),
        Default::default(),
    )
    .expect("Could not load parcels data");

    (lf_farms, lf_parcels)
}

fn create_db(
    lf_farms: LazyFrame,
    lf_parcels: LazyFrame,
    _farms_metadata: &SPSSMetadata,
    _parcels_metadata: &SPSSMetadata,
) -> Result<(), Box<dyn Error>> {
    // Composite key
    let composite_key = ["S101", "S102", "S105", "S106", "S108"];
    let composite_key_parsing = [
        col("S101").cast(DataType::UInt16),
        col("S102").cast(DataType::UInt16),
        col("S105").cast(DataType::UInt32),
        col("S106").cast(DataType::UInt32),
        col("S108").cast(DataType::UInt32),
    ];

    // Transform farms '

    let mut lf_farms = lf_farms
        .with_columns(&composite_key_parsing)
        .select([
            col("S101"),
            col("S102"),
            col("S105"),
            col("S106"),
            col("S108"),
            col("S1273"),
            col("S1274"),
            col("^S1275.*$"),
        ])
        .with_columns([col("S1273").eq(1)])
        .collect()?;

    save_parquet(&mut lf_farms, "farms.parquet")?;

    // Transform parcels
    let mut lf_parcels = lf_parcels
        .with_columns(&composite_key_parsing)
        .group_by(composite_key)
        .agg([
            col("S434").count().alias("total_parcels"),
            col("S434A").sum().alias("mz_annual_crops"),
            col("S434B").sum().alias("mz_permanent_crops"),
            col("S434C").sum().alias("mz_cultivated_pasture"),
            col("S434D").sum().alias("mz_natural_pasture"),
            col("S434E").sum().alias("mz_forest"),
            col("S434F").sum().alias("mz_fallow"),
            col("S434G").sum().alias("mz_infrastructure"),
            col("S434H").sum().alias("mz_unusable"),
        ])
        .with_columns([(col("mz_annual_crops")
            + col("mz_permanent_crops")
            + col("mz_cultivated_pasture")
            + col("mz_natural_pasture")
            + col("mz_forest")
            + col("mz_fallow")
            + col("mz_infrastructure")
            + col("mz_unusable"))
        .alias("total_farm_manzanas")])
        .collect()?;

    save_parquet(&mut lf_parcels, "parcels.parquet")?;

    Ok(())
}

fn save_parquet(df: &mut DataFrame, file_name: &str) -> Result<(), Box<dyn Error>> {
    let file_name = resolve_data_folder(file_name);
    let out_path = PlRefPath::new(file_name);

    let file = File::create(out_path).expect("Failed to create file");

    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Zstd(None))
        .finish(df)?;

    Ok(())
}

fn load_metadata(file_path: &str) -> SPSSMetadata {
    let file = File::open(file_path).expect("Could not find the metadata file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Could not parse the metadata")
}

fn resolve_data_folder(subpath: &str) -> String {
    format!("data/{subpath}")
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load data
    let (lf_farms, lf_parcels) = load_data();

    // Load metadata
    let farms_metadata_path = resolve_data_folder(FARMS_METADATA);
    let farms_metadata = load_metadata(&farms_metadata_path);

    let parcels_metadata_path = resolve_data_folder(PARCELS_METADATA);
    let parcels_metadata = load_metadata(&parcels_metadata_path);

    // create db for analisis
    create_db(lf_farms, lf_parcels, &farms_metadata, &parcels_metadata)?;

    Ok(())
}
