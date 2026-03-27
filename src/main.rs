use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const FARMS_METADATA: &str = "cenagro-2011-explotaciones-agropecuarias-metadata.json";
const PARCELS_METADATA: &str = "cenagro-2011-parcelas-aprovechamiento-tierra-metadata.json";
const FARMS_RAW: &str = "cenagro-2011-explotaciones-agropecuarias.parquet";
const PARCELS_RAW: &str = "cenagro-2011-parcelas-aprovechamiento-tierra.parquet";
const COMPOSITE_KEY: [&str; 5] = ["S101", "S102", "S105", "S106", "S108"];

#[derive(Deserialize, Debug)]
struct SPSSMetadata {
    variable_value_labels: serde_json::Value,
    column_names_to_labels: serde_json::Value,
}

fn load_data(file_name: &str) -> LazyFrame {
    let file_path = resolve_data_folder(file_name);
    let file_path =
        PlRefPath::try_from_pathbuf(file_path).expect("could not parse path to polars format");

    LazyFrame::scan_parquet(file_path, Default::default()).expect("Could not load parcels data")
}

fn transform_farms(lf: LazyFrame, _metadata: &SPSSMetadata) -> LazyFrame {
    let lf = transform_composite_key(lf);
    lf.select([
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
}

fn transform_parcels(lf: LazyFrame, _metadata: &SPSSMetadata) -> LazyFrame {
    let lf = transform_composite_key(lf);
    lf.group_by(COMPOSITE_KEY)
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
}

fn transform_composite_key(lf: LazyFrame) -> LazyFrame {
    let composite_key_parsing = [
        col("S101").cast(DataType::UInt16),
        col("S102").cast(DataType::UInt16),
        col("S105").cast(DataType::UInt32),
        col("S106").cast(DataType::UInt32),
        col("S108").cast(DataType::UInt32),
    ];

    lf.with_columns(composite_key_parsing)
}

fn create_db(
    lf_farms: LazyFrame,
    lf_parcels: LazyFrame,
    farms_metadata: &SPSSMetadata,
    parcels_metadata: &SPSSMetadata,
) -> Result<(), Box<dyn Error>> {
    // Transform farms

    let mut lf_farms = transform_farms(lf_farms, farms_metadata).collect()?;

    save_parquet(&mut lf_farms, &resolve_data_folder("farms.parquet"))?;

    // Transform parcels
    let mut lf_parcels = transform_parcels(lf_parcels, parcels_metadata).collect()?;

    save_parquet(&mut lf_parcels, &resolve_data_folder("parcels.parquet"))?;

    Ok(())
}

fn save_parquet(df: &mut DataFrame, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path).expect("Failed to create file");

    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Zstd(None))
        .finish(df)?;

    Ok(())
}

fn load_metadata(file_path: &PathBuf) -> SPSSMetadata {
    let file = File::open(file_path).expect("Could not find the metadata file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Could not parse the metadata")
}

fn resolve_data_folder<T: AsRef<Path>>(sub_path: T) -> PathBuf {
    let prefix_path = Path::new("data");

    prefix_path.join(sub_path.as_ref())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load data
    let lf_farms = load_data(FARMS_RAW);
    let lf_parcels = load_data(PARCELS_RAW);

    // Load metadata
    let farms_metadata_path = resolve_data_folder(FARMS_METADATA);
    let farms_metadata = load_metadata(&farms_metadata_path);

    let parcels_metadata_path = resolve_data_folder(PARCELS_METADATA);
    let parcels_metadata = load_metadata(&parcels_metadata_path);

    // create db for analisis
    create_db(lf_farms, lf_parcels, &farms_metadata, &parcels_metadata)?;

    Ok(())
}
