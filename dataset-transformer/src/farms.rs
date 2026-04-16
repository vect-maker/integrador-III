use crate::composite_key::{COMPOSITE_KEY_CODES, transform_composite_key};
use polars::prelude::*;

pub fn transform_farms(lf: LazyFrame) -> LazyFrame {
    let mut cols_to_select: Vec<Expr> = COMPOSITE_KEY_CODES.into_iter().map(col).collect();

    cols_to_select.extend([
        // Load source
        col("^S1275.$"),
        // load requested loan
        col("^S1274A.$"),
        col("^S1274B.$"),
        // parcel area
        col("S427"),
        col("S428"),
        // hired workers
        col("S1067"),
        // total of temp and permanent workers
        col("S1068A"),
        col("S1069A"),
        // has irrigation system
        col("S538"),
        // Tracción Animal vs. Mecanizad
        col("S648A"),
        col("S648B"),
        // Demographics
        col("S211D"),
        col("S322"),
        // main activity
        col("S324"),
    ]);

    let lf = lf.select(cols_to_select);
    // transform composite key
    let lf = transform_composite_key(lf);
    // cas parcels area columns
    let lf = lf
        .with_columns([
            col("S427").cast(DataType::Float32).alias("total_area_mz"),
            col("S428").cast(DataType::Float32).alias("total_area_sqm"),
        ])
        .drop(cols(["S427", "S428"]));

    // create loan colums
    let lf = lf
        .with_columns([
            col("S1275A").is_not_null().alias("loan_banco"),
            col("S1275B").is_not_null().alias("loan_banco_produzcamos"),
            col("S1275C").is_not_null().alias("loan_ong"),
            col("S1275D").is_not_null().alias("loan_cooperativa"),
            col("S1275E").is_not_null().alias("loan_gobierno"),
            col("S1275F").is_not_null().alias("loan_comercial"),
            col("S1275G").is_not_null().alias("loan_prestamista"),
            col("S1275H").is_not_null().alias("loan_acopiador"),
            col("S1275I").is_not_null().alias("loan_otro"),
        ])
        .drop(cols([
            "S1275A", "S1275B", "S1275C", "S1275D", "S1275E", "S1275F", "S1275G", "S1275H",
            "S1275I",
        ]));

    let lf = lf.with_columns([any_horizontal([col("^loan_.*$")])
        .expect("Could not creae has any loan column")
        .alias("has_any_loan")]);

    // create receive and requested column
    let lf = lf
        .with_columns([
            // Series A: Solicitó (Financial Demand)
            col("S1274A1")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("req_crop"),
            col("S1274A2")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("req_livestock"),
            col("S1274A3")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("req_aquaculture"),
            col("S1274A4")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("req_forestry"),
            // Series B: Recibió (Financial Inclusion)
            col("S1274B1")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("rec_crop"),
            col("S1274B2")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("rec_livestock"),
            col("S1274B3")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("rec_aquaculture"),
            col("S1274B4")
                .eq(lit(1.0))
                .fill_null(false)
                .alias("rec_forestry"),
        ])
        .drop(cols([
            "S1274A1", "S1274A2", "S1274A3", "S1274A4", "S1274B1", "S1274B2", "S1274B3", "S1274B4",
        ]));

    let lf = lf.with_columns([
        any_horizontal([col("^req_.*$")])
            .expect("Could not create requested_loan")
            .alias("requested_loan"),
        any_horizontal([col("^rec_.*$")])
            .expect("Could not create received_loan")
            .alias("received_loan"),
    ]);

    // labor matrix
    let lf = lf
        .with_columns([col("S1067").eq(lit(1.0)).alias("hired_workers")])
        .drop(cols(["S1067"]));

    // total temp and permanent workers

    let lf = lf
        .with_columns([
            col("S1068A")
                .fill_null(0)
                .cast(DataType::UInt16)
                .alias("permanent_workers_total"),
            col("S1069A")
                .fill_null(0)
                .cast(DataType::UInt16)
                .alias("temporal_workers_total"),
        ])
        .drop(cols(["S1068A", "S1069A"]));
    // has irrigation sytem
    let lf = lf
        .with_columns([col("S538").eq(lit(1.0)).alias("has_irrigation_system")])
        .drop(cols(["S538"]));

    // animal vs mechanized
    let lf = lf
        .with_columns([
            col("S648A").eq(lit(1.0)).alias("traction_animal"),
            col("S648B").eq(lit(2.0)).alias("traction_tractor"),
        ])
        .drop(cols(["S648A", "S648B"]));

    // add producer
    let category_names = ["hombre", "mujer", "ignorado"];
    let producer_gender_old = lit(Series::new("keys".into(), &[1, 2, 9]));
    let producer_gender_new = lit(Series::new("vals".into(), category_names));

    let gender_categories = FrozenCategories::new(category_names.to_vec())
        .expect("Failed to create FrozenCategories of producer gender");

    let gender_structure_dtype = DataType::Enum(
        gender_categories.clone(),
        gender_categories.mapping().clone(),
    );
    let lf: LazyFrame = lf
        .with_columns([col("S211D")
            .cast(DataType::UInt8)
            .replace_strict(
                producer_gender_old,
                producer_gender_new,
                Some(lit("ignorado")),
                Some(DataType::String),
            )
            .cast(gender_structure_dtype)
            .alias("producer_gender")])
        .drop(cols(["S211D"]));

    // add operation structure
    let category_names = [
        "individual",
        "cooperativa",
        "colectivo_familiar",
        "empresa",
        "comunidad_indigena",
        "administracion_publica",
        "otra",
        "ignorado",
    ];

    let work_structure_old = lit(Series::new("keys".into(), &[1, 2, 3, 4, 5, 6, 7, 9]));
    let work_structure_new = lit(Series::new("vals".into(), &category_names));

    let operational_categories = FrozenCategories::new(category_names.to_vec())
        .expect("Failed to create FrozenCategories of operational_structure");

    let operational_structure_dtype = DataType::Enum(
        operational_categories.clone(),
        operational_categories.mapping().clone(),
    );

    let lf: LazyFrame = lf
        .with_columns([col("S322")
            .cast(DataType::UInt8)
            .replace_strict(
                work_structure_old,
                work_structure_new,
                Some(lit("ignorado")),
                Some(DataType::String),
            )
            .cast(operational_structure_dtype)
            .alias("farm_operational_structure")])
        .drop(cols(["S322"]));
    // add activities
    let activity_category_names = [
        "autoconsumo",
        "mercado_interno",
        "exportacion_tradicional",
        "exportacion_no_tradicional",
        "otros_agricolas",
        "ganaderia_leche",
        "ganaderia_carne",
        "doble_proposito",
        "ganaderia_menor",
        "crianza",
        "otros_pecuarios",
        "acuicola",
        "forestal",
        "apicola",
        "abandono_inactiva",
        "ignorado",
    ];

    let principal_activity_old = lit(Series::new(
        "keys".into(),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 99],
    ));
    let principal_activity_new = lit(Series::new("vals".into(), &activity_category_names));

    let activity_categories = FrozenCategories::new(activity_category_names.to_vec())
        .expect("Failed to create FrozenCategories of principal activity");

    let activity_structure_dtype = DataType::Enum(
        activity_categories.clone(),
        activity_categories.mapping().clone(),
    );

    lf.with_columns([col("S324")
        .cast(DataType::UInt8)
        .replace_strict(
            principal_activity_old,
            principal_activity_new,
            Some(lit("ignorado")),
            Some(DataType::String),
        )
        .cast(activity_structure_dtype)
        .alias("principal_activity")])
        .drop(cols(["S324"]))
}
