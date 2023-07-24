//! # BuiltinVariable
//!
use std::collections;
use std::fmt;
use crate::{InnerError, InnerResult};
use super::{Node, VariableEval, VariableName};

pub struct BuiltinVariable {
    pub eval: VariableEval,
    pub name: VariableName, 
}

impl BuiltinVariable {
    // TODO: 
    //
    // * Add:
    //   * colref
    //   * colleft
    //   * colright
    pub fn all() -> collections::HashMap<String, BuiltinVariable> {
        let mut vars = collections::HashMap::new();

        // `colnum` - The number of the current column.  
        vars = Self::def_var(vars, "colnum", |a1| {
            if let Some(x) = a1.x() {
                Ok(Node::Integer((x as i64) + 1))
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a column component"))
            }
        });

        // `cellref` - A reference to the current cell.  
        vars = Self::def_var(vars, "cellref", |a1| {
            Ok(Node::Reference(a1.to_string()))
        });

        // `rowabove` - A (row-relative) reference to the row above the current cell.
        vars = Self::def_var(vars, "rowabove", |a1| {
            if let Some(y) = a1.y() {
                let a1_above = a1_notation::A1::builder().y((y - 1).max(0)).build()?;
                Ok(Node::Reference(a1_above.to_string()))
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });
        
        // `rowbelow` - A (row-relative) reference to the row below the current cell.
        vars = Self::def_var(vars, "rowbelow", |a1| {
            if let Some(y) = a1.y() {
                let a1_below = a1_notation::A1::builder().y(y + 1).build()?;
                Ok(Node::Reference(a1_below.to_string()))
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });
        
        // `rownum` - The number of the current row.  Starts at 1.
        vars = Self::def_var(vars, "rownum", |a1| {
            if let Some(y) = a1.y() {
                Ok(Node::Integer((y as i64) + 1))
            } else {
                Err(InnerError::bad_input(
                    &a1.to_string(),
                    "Expected a cell reference with a row component"))
            }
        });

        // `rowref` - A reference to the current row.  
        vars = Self::def_var(vars, "rowref", |a1| {
            if let Some(y) = a1.y() {
                let row_a1 = a1_notation::A1::builder().y(y).build()?;
                Ok(Node::Reference(row_a1.to_string()))
            } else {
                Err(InnerError::bad_input(
                        &a1.to_string(), 
                        "Expected a cell reference with a row component"))
            }
        });

        vars
    }

    fn def_var<F>(
        mut vars: collections::HashMap<String, BuiltinVariable>, 
        name: &str, 
        eval: F,
    ) -> collections::HashMap<String, BuiltinVariable>
    where F: Fn(&a1_notation::A1) -> InnerResult<Node> + 'static {
        vars.insert(
            name.to_string(), 
            Self {
                name: name.to_string(),
                eval: Box::new(eval),
            });

        vars
    }
}

impl fmt::Debug for BuiltinVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinVariable")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

