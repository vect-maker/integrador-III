use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;
mod composite_key;
mod env;
mod farms;
mod parcels;

fn load_data(file_name: &str) -> LazyFrame {
    let file_path = resolve_data_folder(file_name);
    let file_path =
        PlRefPath::try_from_pathbuf(file_path).expect("could not parse path to polars format");

    LazyFrame::scan_parquet(file_path, Default::default()).expect("Could not load parcels data")
}

fn save_parquet(df: &mut DataFrame, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path).expect("Failed to create file");

    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Zstd(None))
        .finish(df)?;

    Ok(())
}

fn resolve_data_folder<T: AsRef<Path>>(path: T) -> PathBuf {
    let prefix_path = Path::new("");

    prefix_path.join(path.as_ref())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting processing of data");
    let start = Instant::now();

    // load env varibles
    let app_env = env::load_config();

    // Load data
    let lf_farms = load_data(&app_env.farms_path);
    let lf_parcels = load_data(&app_env.parcels_path);

    // Transform farms
    let mut lf_farms = farms::transform_farms(lf_farms).collect()?;
    save_parquet(&mut lf_farms, &resolve_data_folder("data/farms.parquet"))?;

    // Transform parcels
    let mut lf_parcels = parcels::transform_parcels(lf_parcels).collect()?;
    save_parquet(
        &mut lf_parcels,
        &resolve_data_folder("data/parcels.parquet"),
    )?;

    println!("Finished processing");
    println!("Pipeline execution time: {:?}", start.elapsed());
    Ok(())
}
