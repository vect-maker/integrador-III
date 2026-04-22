use datafusion::error::Result;
use datafusion::prelude::*;

pub trait DataFrameExt {
    fn with_columns(self, exprs: Vec<Expr>) -> Result<DataFrame>;
}

impl DataFrameExt for DataFrame {
    fn with_columns(self, exprs: Vec<Expr>) -> Result<DataFrame> {
        let mut projection: Vec<Expr> = self
            .schema()
            .fields()
            .iter()
            .map(|f| col(f.name()))
            .collect();

        projection.extend(exprs);
        self.select(projection)
    }
}
