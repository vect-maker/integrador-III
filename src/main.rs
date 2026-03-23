use polars::prelude::*;
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let (lf_farms, lf_parcels) = load_data();
    let lf_head = lf_farms.limit(5).collect()?;
    println!("{lf_head:?}");

    Ok(())
}
