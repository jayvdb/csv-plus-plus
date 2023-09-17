use crate::{Error, Template, Result};
use crate::target::{file_backer_upper, merge_rows, CompilationTarget, Csv, MergeResult};

impl CompilationTarget for Csv<'_> {
    fn write_backup(&self) -> Result<()> {
        file_backer_upper::backup_file(&self.path)?;
        Ok(())
    }

    fn write(&self, template: &Template) -> Result<()> {
        let existing_values = Self::read(&self.path, &self.runtime.output)?;

        let new_values = template.spreadsheet.borrow();
        let widest_row = new_values.widest_row();

        let mut writer = csv::WriterBuilder::new()
            .flexible(true)
            .from_path(&self.path).map_err(|e|
                Error::TargetWriteError {
                    message: format!("Unable to open output file for writing: {:?}", e),
                    output: self.runtime.output.clone(),
                })?;

        for (index, row) in new_values.cells.iter().enumerate() {
            let mut output_row: Vec<String> = merge_rows(
                    existing_values.cells.get(index).unwrap_or(&vec![].to_owned()), 
                    row, 
                    &self.runtime.options,
                )
                .iter()
                .map(|cell| {
                    match cell {
                        MergeResult::New(v) => v.to_string(),
                        MergeResult::Existing(v) => v.to_string(),
                        MergeResult::Empty => "".to_owned(),
                    }
                })
                .collect();

            // all rows have to be as wide as the widest row
            output_row.resize(widest_row, "".to_string());
            
            writer.write_record(output_row).map_err(|e|
                Error::TargetWriteError { 
                    message: format!("Unable to write row {index}: {e}"),
                    output: self.runtime.output.clone(),
                })?;
        }

        writer.flush().map_err(|e|
            Error::TargetWriteError {
                message: format!("Unable to finish writing to output: {}", e),
                output: self.runtime.output.clone(),
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO more tests
}
