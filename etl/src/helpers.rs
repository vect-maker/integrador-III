use anyhow::{Context, Result};
use datafusion::prelude::*;

pub fn any_horizontal(cols: &[&str]) -> Result<Expr> {
    cols.iter()
        .map(|&name| coalesce(vec![col(name), lit(false)]))
        .reduce(|acc, c| acc.or(c))
        .context("Column list for horizontal aggregation cannot be empty")
}
