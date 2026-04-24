use crate::mappings::composite_key::COMPOSITE_KEY;
use crate::saver;
use crate::transformers;
use anyhow::Result;
use datafusion::prelude::*;

pub async fn run_dataset_join_pipeline(
    _ctx: &SessionContext,
    farms_df: DataFrame,
    parcels_df: DataFrame,
    out_dir: &str,
) -> Result<DataFrame> {
    let parcels_df = transformers::parcels::aggregate_parcels_by_composite_key(parcels_df).await?;

    let joined_df = farms_df.alias("farms")?.join(
        parcels_df.alias("parcels")?,
        JoinType::Inner,
        &COMPOSITE_KEY,
        &COMPOSITE_KEY,
        None,
    )?;

    let projection: Vec<Expr> = joined_df
        .schema()
        .iter() // Yields (Option<&TableReference>, &FieldRef)
        .filter_map(|(qualifier, field)| {
            let name = field.name();
            let q_str = qualifier.map(|q| q.to_string());

            match q_str {
                // Keep all columns from the 'farms' table
                Some(q) if q == "farms" => Some(col(format!("{}.{}", q, name)).alias(name)),
                // Keep only metric columns from the 'parcels' table
                Some(q) if q == "parcels" && !COMPOSITE_KEY.contains(&name.as_str()) => {
                    Some(col(format!("{}.{}", q, name)).alias(name))
                }
                _ => None,
            }
        })
        .collect();

    let df = joined_df.select(projection)?;

    saver::save_data(df.clone(), &format!("{}/{}", out_dir, "farms.parquet")).await?;

    Ok(df)
}
