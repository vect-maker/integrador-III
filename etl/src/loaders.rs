use crate::mappings::composite_key::COMPOSITE_KEY_CODES;
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

pub async fn load_farms_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = load_data(ctx, file)
        .await
        .context("Failed to load farms data")?;
    let mut cols_to_select: Vec<Expr> = COMPOSITE_KEY_CODES.into_iter().map(col).collect();

    cols_to_select.extend([
        // --- CREDIT SOURCES ---
        col("s1275a"), // Banco/Microfinanciera
        col("s1275b"), // Banco Produzcamos
        col("s1275c"), // ONG
        col("s1275d"), // Cajas Rurales y Cooperativas Ay C
        col("s1275e"), // Proyectos/Programas de Gobierno
        col("s1275f"), // Empresa/Casa Comercial
        col("s1275g"), // Prestamista
        col("s1275h"), // Acopiador
        col("s1275i"), // Otro
        // --- LOAN REQUESTS AND RECEIPTS ---
        col("s1274a1"), // Solicitó Préstamo Agrícola
        col("s1274b1"), // Recibió Préstamo Agrícola
        col("s1274a2"), // Solicitó Préstamo Pecuario
        col("s1274b2"), // Recibió Préstamo Pecuario
        col("s1274a3"), // Solicitó Préstamo Acuícola
        col("s1274b3"), // Recibió Préstamo Acuícola
        col("s1274a4"), // Solicitó Préstamo Forestal
        col("s1274b4"), // Recibió Préstamo Forestal
        // --- LAND SCALING ---
        col("s427"), // Cuántas Parcelas tiene en este municipio
        col("s428"), // Superficie Total de las Parcelas de la EA
        // --- EMPLOYMENT ---
        col("s1067"),  // Contrató Trabajadores en EA
        col("s1068a"), // Total Trabajadores Permanentes
        col("s1069a"), // Total Trabajadores Temporales
        // --- IRRIGATION ---
        col("s538"), // Tiene Sistema de riego en EA
        // --- TECHNOLOGY AND TRACTION ---
        col("s648a"), // Animales de Tiro/Yunta
        col("s648b"), // Tractor
        // --- PRODUCER AND ACTIVITY ---
        col("s211d"), // Sexo del productor
        col("s322"),  // Forma de trabajar EA
        col("s324"),  // Actividad Principal de la EA
    ]);

    let df = df
        .select(cols_to_select)
        .context("Could not select all required columns")?;

    Ok(df)
}

pub async fn load_parcels_data(ctx: &SessionContext, file: &str) -> Result<DataFrame> {
    let df = load_data(ctx, file)
        .await
        .context("Failed to load farms data")?;

    let mut cols_to_select: Vec<Expr> = COMPOSITE_KEY_CODES.into_iter().map(col).collect();

    cols_to_select.extend([
        // --- LAND USE AND APROVECHAMIENTO ---
        col("s434"),  // N° Parcela
        col("s434a"), // Cult anuales/temporales
        col("s434b"), // Cult permanentes/semiperm
        col("s434c"), // Pastos cultivados/sembrados
        col("s434d"), // Pastos naturales
        col("s434e"), // Tierras descanso/tacotales
        col("s434f"), // Bosques
        col("s434g"), // Instalaciones y viales
        col("s434h"), // Pantanos/pedregales/otras
    ]);

    Ok(df)
}
