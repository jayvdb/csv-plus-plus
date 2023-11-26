mod display;

use crate::{
    BorderSide, BorderStyle, Cell, Compiler, DataValidation, Fill, HorizontalAlign, NumberFormat,
    Result, Rgb, TextFormat, VerticalAlign,
};
use std::collections::HashSet;

type CsvRowResult = std::result::Result<csv::StringRecord, csv::Error>;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub border_color: Option<Rgb>,
    pub border_style: Option<BorderStyle>,
    pub borders: HashSet<BorderSide>,
    pub color: Option<Rgb>,
    pub data_validation: Option<DataValidation>,
    pub fill: Option<Fill>,
    pub font_color: Option<Rgb>,
    pub font_family: Option<String>,
    pub font_size: Option<u8>,
    pub horizontal_align: Option<HorizontalAlign>,
    pub lock: bool,
    pub note: Option<String>,
    pub number_format: Option<NumberFormat>,
    pub text_formats: HashSet<TextFormat>,
    pub var: Option<String>,
    pub vertical_align: Option<VerticalAlign>,
}

impl Row {
    pub(crate) fn parse(
        record_result: CsvRowResult,
        row_a1: a1_notation::Row,
        compiler: &Compiler,
    ) -> Result<Self> {
        let mut row = Self::default();

        // handle if the row is blank or an error or something. (maybe we should warn here if it's
        // an error?)
        let csv_parsed_row = &record_result.unwrap_or_default();

        for (cell_index, unparsed_value) in csv_parsed_row.into_iter().enumerate() {
            let cell_a1 = a1_notation::Address::new(cell_index, row_a1.y);
            let cell = Cell::parse(unparsed_value, cell_a1, &mut row, compiler)?;
            row.cells.push(cell);
        }

        Ok(row)
    }

    pub(crate) fn clone_to_row(&self, new_row: a1_notation::Row) -> Self {
        Self {
            fill: self.fill.map(|f| f.clone_to_row(new_row)),
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn parse() {
        // TODO
    }

    #[test]
    fn clone_to_row() {
        let row = Row {
            cells: vec![Cell {
                value: "foo".to_string(),
                ..Default::default()
            }],
            fill: Some(Fill::new(22, Some(100))),
            ..Default::default()
        };

        assert_eq!(
            row.clone_to_row(5.into()),
            Row {
                cells: vec![Cell {
                    value: "foo".to_string(),
                    ..Default::default()
                }],
                fill: Some(Fill::new(5, Some(100))),
                ..Default::default()
            }
        );
    }
}
