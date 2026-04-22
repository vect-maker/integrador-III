use anyhow::{Context, Result};
use datafusion::common::Column;
use datafusion::prelude::*;

pub async fn load_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = ctx
        .read_parquet(file, ParquetReadOptions::default())
        .await
        .context("Failed to scan parquet")?;

    let projection = df
        .schema()
        .fields()
        .iter()
        .map(|f| {
            let source_name = f.name();
            let source_col = Column::new(None::<String>, source_name);
            Expr::Column(source_col).alias(source_name.to_lowercase())
        })
        .collect::<Vec<_>>();

    let df = df
        .select(projection)
        .context("Failed to normalize the column names")?;

    Ok(df)
}
