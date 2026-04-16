use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub farms_path: String,
    pub parcels_path: String,
    pub farms_meta_path: String,
    pub parcels_meta_path: String,
    pub out_dir: String,
}

pub fn load_config() -> AppConfig {
    let app = envy::from_env::<AppConfig>()
        .expect("❌ CRITICAL: Failed to parse analytical environment configuration");

    validate_not_empty(&[
        ("FARMS_PATH", &app.farms_path),
        ("PARCELS_PATH", &app.parcels_path),
        ("FARMS_META_PATH", &app.farms_meta_path),
        ("PARCELS_META_PATH", &app.parcels_meta_path),
        ("OUT_DIR", &app.out_dir),
    ]);

    app
}

fn validate_not_empty(vars: &[(&str, &String)]) {
    for (name, value) in vars {
        if value.is_empty() {
            panic!(
                "❌ CRITICAL: Environment variable {} is defined but empty",
                name
            );
        }
    }
}
