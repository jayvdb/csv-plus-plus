//! It's common that we want to create errors with references to data that the `SourceCode` owns.
//! In which case it makes sense to add some helper functions to do that
use super::{CharOffset, LineNumber, SourceCode};
use crate::error::{BadInput, ParseError};
use crate::Error;
use std::cmp;

// how many lines above (and below) we'll show as context when highlighting error messages
const LINES_IN_ERROR_CONTEXT: LineNumber = 3;

impl SourceCode {
    pub(crate) fn code_syntax_error(&self, parse_error: ParseError) -> Error {
        Error::CodeSyntaxError {
            filename: self.filename.clone(),
            parse_error: Box::new(parse_error),
        }
    }

    pub(crate) fn modifier_syntax_error(
        &self,
        parse_error: ParseError,
        position: a1_notation::Address,
    ) -> Error {
        Error::ModifierSyntaxError {
            filename: self.filename.clone(),
            parse_error: Box::new(parse_error),
            position,
        }
    }

    pub(crate) fn eval_error(&self, message: &str, position: a1_notation::Address) -> Error {
        Error::EvalError {
            message: message.to_string(),
            filename: self.filename.clone(),
            position,
        }
    }

    // TODO: can this take an owned BadInput instead? I think most callers would be fine with it
    // and there would be less cloning
    pub(crate) fn parse_error(&self, bad_input: impl BadInput, message: &str) -> ParseError {
        self.parse_error_with_possible_values(bad_input, message, None)
    }

    pub(crate) fn parse_error_with_possible_values(
        &self,
        bad_input: impl BadInput,
        message: &str,
        // TODO: make this a slice
        possible_values: Option<Vec<String>>,
    ) -> ParseError {
        let line_number = bad_input.line_number();
        let line_offset = bad_input.line_offset();
        let highlighted_lines = self.highlight_line(line_number, line_offset);

        ParseError {
            bad_input: bad_input.to_string(),
            highlighted_lines,
            message: message.to_string(),
            line_number,
            line_offset,
            possible_values: possible_values
                .clone()
                .map(|pvs| pvs.iter().map(|pv| pv.to_string()).collect::<Vec<String>>()),
        }
    }

    /// Given a line number and character offset, return an array of `String`s that can be rendered
    /// for a friendly message for debugging (that highlights the line and character in question).
    fn highlight_line(&self, line_number: LineNumber, line_offset: CharOffset) -> Vec<String> {
        let lines = self
            .original
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        // are they requesting a line totally outside of the range?
        if line_number >= lines.len() {
            return vec![];
        }

        let start_index = line_number.saturating_sub(LINES_IN_ERROR_CONTEXT);
        let end_index = cmp::min(lines.len(), line_number + LINES_IN_ERROR_CONTEXT + 1);

        // start with 3 lines before, and also include our highlight line
        let mut lines_out = lines[start_index..(line_number + 1)].to_vec();

        // save the number of this line because we want to skip line-numbering it below
        let skip_numbering_on = lines_out.len();

        // draw something like this to highlight it:
        // ```
        //      foo!
        // --------^
        // ```
        lines_out.push(format!("{}^", "-".repeat(line_offset)));

        // and 3 lines after
        lines_out.append(&mut lines[(line_number + 1)..end_index].to_vec());

        // now format each line with line numbers
        let longest_line_number = (line_number + LINES_IN_ERROR_CONTEXT).to_string().len();
        let mut line_count = line_number.saturating_sub(LINES_IN_ERROR_CONTEXT);

        // now iterate over it and apply lines numbers like `XX: some_code( ...` where XX is the
        // line number
        lines_out
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // don't increment the line *after* the line we're highlighting.  because it's the
                // ----^ thing and it doesn't correspond to a source code row, it's highlighting the
                // text above it
                if i == skip_numbering_on {
                    format!(" {: <width$}: {}", " ", line, width = longest_line_number)
                } else {
                    line_count += 1;
                    format!(
                        " {: <width$}: {}",
                        line_count,
                        line,
                        width = longest_line_number
                    )
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn highlight_line() {
        let source_code = SourceCode::new(
            "
# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(
            source_code.highlight_line(7, 5),
            vec![
                " 5 : other_var := 42",
                " 6 : ",
                " 7 : something {",
                " 8 :     foo: bar",
                "   : -----^",
                " 9 : }",
                " 10: ---",
                " 11: foo,bar,baz",
            ]
        );
    }

    #[test]
    fn highlight_line_at_top() {
        let source_code = SourceCode::new(
            "# A comment

var := 1
other_var := 42

something {
    foo: bar
}
---
foo,bar,baz
            ",
            path::PathBuf::from("test.csvpp"),
        )
        .unwrap();

        assert_eq!(
            source_code.highlight_line(0, 5),
            vec![
                " 1: # A comment",
                "  : -----^",
                " 2: ",
                " 3: var := 1",
                " 4: other_var := 42",
            ]
        );
    }
}