use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub farms_path: String,
    pub parcels_path: String,
    pub out_dir: String,
}

pub fn load_config() -> Result<AppConfig> {
    let farms_path = env::var("FARMS_PATH").context("Missing FARMS_PATH")?;
    let parcels_path = env::var("PARCELS_PATH").context("Missing PARCELS_PATH")?;
    let out_dir = env::var("OUT_DIR").context("Missing OUT_DIR")?;

    let app_config = AppConfig {
        farms_path,
        parcels_path,
        out_dir,
    };

    Ok(app_config)
}
