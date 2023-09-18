//! # Cell
//!
use a1_notation::{Address, Row};
use serde::{Deserialize, Serialize};
use crate::{Modifier, Result, RowModifier, SourceCode};
use crate::ast::Ast;
use crate::compiler::ast_parser::AstParser;
use crate::compiler::modifier_parser::ModifierParser;

mod display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Cell {
    pub ast: Option<Ast>,
    pub position: Address,
    pub modifier: Modifier,
    pub value: String,
}

impl Cell {
    pub fn parse(
        input: &str,
        position: Address,
        row_modifier: &RowModifier,
        source_code: &SourceCode,
    ) -> Result<(Self, Option<RowModifier>)> {
        let parsed_modifiers = ModifierParser::parse(input, source_code, position, row_modifier)?;
        let cell = Self {
            ast: Self::parse_ast(&parsed_modifiers.value, source_code)?,
            position,
            modifier: parsed_modifiers.modifier
                .unwrap_or_else(|| parsed_modifiers.row_modifier.clone()
                                .unwrap_or(row_modifier .clone())
                                .into()),
            value: parsed_modifiers.value,
        };

        Ok((cell, parsed_modifiers.row_modifier))
    }

    /// Clone the `Cell` keeping all of it's data the same, except it will reflect that it's been 
    /// moved to `new_row`.  This involves updating `position` and `expand.start_row`
    pub fn clone_to_row(&self, new_row: Row) -> Self {
        Self {
            position: Address::new(self.position.column.x, new_row.y),
            ..self.clone()
        }
    }

    fn parse_ast(input: &str, source_code: &SourceCode) -> Result<Option<Ast>> {
        if let Some(without_equals) = input.strip_prefix('=') {
            Ok(Some(AstParser::parse(without_equals, false, Some(source_code))?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::TestFile;
    use crate::*;
    use super::*;

    #[test]
    fn clone_to_row() {
        let cell = Cell {
            ast: None,
            value: "foo".to_string(),
            position: Address::new(1, 4),
            modifier: Modifier::default(),
        };

        let cloned_row = cell.clone_to_row(10.into());
        assert_eq!(cloned_row.position.to_string(), "B11");
    }

    #[test]
    fn parse_no_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let (cell, _) = Cell::parse("foo", 
                               Address::new(0, 4),
                               &RowModifier::default(),
                               &source_code).unwrap();

        assert_eq!(cell.value, "foo");
        assert_eq!(cell.ast, None);
    }

    #[test]
    fn parse_ast() {
        let test_file = TestFile::new("csv", "foo,bar,baz\n1,2,3\n");
        let source_code = test_file.into();
        let (cell, _) = Cell::parse("=1 + foo", 
                               Address::new(0, 4),
                               &RowModifier::default(),
                               &source_code).unwrap();

        assert!(cell.ast.is_some());
    }
}
