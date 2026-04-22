use datafusion::config::TableParquetOptions;
use datafusion::prelude::*;

pub async fn save_data(df: DataFrame, path: &str) -> anyhow::Result<()> {
    let mut table_options = TableParquetOptions::default();
    table_options.global.compression = Some("zstd(3)".to_string());

    df.write_parquet(path, Default::default(), Some(table_options))
        .await?;

    Ok(())
}
