use datafusion::error::Result;
use datafusion::prelude::*;
use std::collections::HashMap;

pub trait DataFrameExt {
    fn with_columns(self, exprs: Vec<Expr>) -> Result<DataFrame>;
}

impl DataFrameExt for DataFrame {
    fn with_columns(self, exprs: Vec<Expr>) -> Result<DataFrame> {
        let mut updates = HashMap::new();
        for e in exprs {
            let name = e.name_for_alias()?;
            updates.insert(name, e);
        }

        let mut projection: Vec<Expr> = self
            .schema()
            .fields()
            .iter()
            .map(|f| {
                if let Some(new_expr) = updates.remove(f.name()) {
                    new_expr
                } else {
                    col(f.name())
                }
            })
            .collect();

        let mut remaining: Vec<_> = updates.into_iter().collect();
        remaining.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, new_col) in remaining {
            projection.push(new_col);
        }

        self.select(projection)
    }
}
