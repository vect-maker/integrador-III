use duckdb::{self, params};
use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

const DB_NAME: &str = "cenagro.duckdb";
const FARMS_METADATA: &str = "cenagro-2011-explotaciones-agropecuarias-metadata.json";
const PARCELS_METADATA: &str = "cenagro-2011-parcelas-aprovechamiento-tierra-metadata.json";

#[derive(Deserialize)]
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

fn create_db(duck_conn: duckdb::Connection) {}

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
    let lf_head = lf_farms.limit(5).collect()?;

    // Load metadata
    let farms_metadata_path = resolve_data_folder(FARMS_METADATA);
    let farms_metadata = load_metadata(&farms_metadata_path);

    let parcels_metadata_path = resolve_data_folder(PARCELS_METADATA);
    let parcels_metadata = load_metadata(&parcels_metadata_path);

    // load duck db config
    let duck_conn = duckdb::Connection::open(format!("data/{DB_NAME}"))?;
    let config = fs::read_to_string("sql/config.sql")?;
    duck_conn.execute_batch(&config)?;

    // create db for analisis
    create_db(duck_conn);

    println!("{lf_head:?}");

    Ok(())
}
