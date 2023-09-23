//! # Expand
//!
use a1_notation::Row;
use serde::{Deserialize, Serialize};
use std::cmp;

mod into;

const ROW_MAX: usize = 1000;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Expand {
    pub amount: Option<usize>,
    pub start_row: Row,
}

impl Expand {
    pub fn clone_to_row<R: Into<Row>>(&self, row: R) -> Self {
        Self {
            amount: self.amount,
            start_row: row.into(),
        }
    }

    pub fn end_row(&self) -> Row {
        if let Some(a) = self.amount {
            cmp::min(self.start_row.y + a, ROW_MAX).into()
        } else {
            ROW_MAX.into()
        }
    }

    pub fn expand_amount<R: Into<Row>>(&self, row: R) -> usize {
        self.amount.unwrap_or(ROW_MAX - row.into().y)
    }

    pub fn new<R: Into<Row>>(start_row: R, amount: Option<usize>) -> Self {
        Self {
            amount,
            start_row: start_row.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_to_row() {
        let expand = Expand {
            amount: Some(10),
            start_row: 0.into(),
        };

        assert_eq!(
            expand.clone_to_row(55),
            Expand {
                amount: Some(10),
                start_row: 55.into()
            }
        );
    }

    #[test]
    fn end_row_with_amount() {
        let expand = Expand {
            amount: Some(10),
            start_row: 0.into(),
        };

        assert_eq!(expand.end_row(), 10.into());
    }

    #[test]
    fn end_row_without_amount() {
        let expand = Expand {
            amount: None,
            start_row: 0.into(),
        };

        assert_eq!(expand.end_row(), 1000.into());
    }

    #[test]
    fn expand_amount_finite() {
        let expand = Expand {
            amount: Some(5),
            start_row: 0.into(),
        };

        assert_eq!(expand.expand_amount(0), 5);
        assert_eq!(expand.expand_amount(10), 5);
    }

    #[test]
    fn expand_amount_infinite() {
        let expand = Expand {
            amount: None,
            start_row: 0.into(),
        };

        assert_eq!(expand.expand_amount(0), 1000);
        assert_eq!(expand.expand_amount(10), 990);
    }
}