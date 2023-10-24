//! # ModifierLexer
//!
//! This is the lexer/tokenizer used for parsing csv++ modifiers - it's a little different than
//! most parsers which parse their entire input into tokens in one go. This tokenizes as the
//! parser goes since it is context-dependent.
//!
//! [https://en.wikipedia.org/wiki/Lexer_hack](See also: Lexer hack)
//!
//!
// TODO:
// * need to lowercase the input but we can't do it on the entire value because we don't want to
//     lowercase the stuff outside the modifier definition
//
// * make the `take_date` parser more strict
use super::TokenMatcher;
use crate::error::{BadInput, ParseError, ParseResult};
use crate::{CharOffset, Runtime};

mod token;
mod token_library;
mod token_match;
mod unknown_token;

pub(crate) use token::Token;
pub(crate) use token_library::TokenLibrary;
pub(crate) use token_match::TokenMatch;
pub(crate) use unknown_token::UnknownToken;

#[derive(Debug)]
pub(crate) struct ModifierLexer<'a> {
    cell_offset: CharOffset,
    input: &'a str,
    position: a1_notation::Address,
    runtime: &'a Runtime,
}

impl<'a> ModifierLexer<'a> {
    pub(super) fn new(
        input: &'a str,
        position: a1_notation::Address,
        runtime: &'a Runtime,
    ) -> Self {
        Self {
            cell_offset: 0,
            input,
            position,
            runtime,
        }
    }

    /// The rest of the input that has not been consumed
    pub(super) fn rest(&self) -> String {
        self.input.trim().to_string()
    }

    pub(super) fn maybe_take_start_modifier(&mut self) -> Option<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix("[[") {
            let token_match = self.match_token(Token::StartCellModifier, "[[");
            self.replace_input(without_match, 2);
            Some(token_match)
        } else if let Some(without_match) = self.input.strip_prefix("![[") {
            let token_match = self.match_token(Token::StartRowModifier, "![[");
            self.replace_input(without_match, 3);
            Some(token_match)
        } else {
            None
        }
    }

    // TODO: this name is kinda misleading since it also takes an equal first
    // maybe just rename to take_identifier
    pub(super) fn take_modifier_right_side(&mut self) -> ParseResult<TokenMatch> {
        self.take_token(Token::Equals)?;
        self.take_token(Token::Identifier)
    }

    pub(super) fn maybe_take_comma(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::Comma, ",")
    }

    pub(super) fn maybe_take_date(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.runtime.cell_token_library.date)
    }

    pub(super) fn maybe_take_equals(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::Equals, "=")
    }

    pub(super) fn maybe_take_number(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.runtime.cell_token_library.number)
    }

    pub(super) fn maybe_take_identifier(&mut self) -> Option<TokenMatch> {
        self.maybe_take_regex(&self.runtime.cell_token_library.identifier)
    }

    pub(super) fn maybe_take_single_quoted_string(&mut self) -> ParseResult<Option<TokenMatch>> {
        Ok(
            if self
                .runtime
                .cell_token_library
                .single_quoted_string
                .try_match(self.input)
                .is_some()
            {
                // self.maybe_take_regex(&self.runtime.cell_token_library.single_quoted_string)
                Some(self.take_single_quoted_string()?)
            } else {
                None
            },
        )
    }

    pub(super) fn maybe_take_slash(&mut self) -> Option<TokenMatch> {
        self.maybe_take(Token::Slash, "/")
    }

    pub(super) fn take_token(&mut self, token: Token) -> ParseResult<TokenMatch> {
        // spaces can be anywhere, so take any leading space
        self.take_whitespace();

        match token {
            Token::A1 => self.take_while(token, |ch| {
                // TODO: make a list of valid A1 characters somewhere
                ch.is_alphanumeric() || ch == '!' || ch == '\'' || ch == ':' || ch == '$'
            }),
            Token::CloseParenthesis => self.take(token, ")"),
            Token::Color => self.take_color(),
            Token::Comma => self.take(token, ","),
            Token::Date => self.take_date(),
            Token::EndModifier => self.take(token, "]]"),
            Token::Equals => self.take(token, "="),
            Token::ModifierName => self.take_while(token, |ch| ch.is_alphanumeric()),
            Token::Identifier => self.take_while(token, |ch| ch.is_alphanumeric() || ch == '_'),
            Token::Number => {
                // TODO: I could do a little better (enforce only one starting - and one .)
                self.take_while(token, |ch| ch.is_ascii_digit() || ch == '-' || ch == '.')
            }
            Token::OpenParenthesis => self.take(token, "("),
            Token::PositiveNumber => self.take_while(token, |ch| ch.is_ascii_digit()),
            Token::String => self.take_string(),
            Token::Slash => self.take(token, "/"),
            Token::StartCellModifier => self.take(token, "[["),
            Token::StartRowModifier => self.take(token, "![["),
        }
    }

    // TODO: can do a little better here, we just take numbers and slashes, but we could be more
    // strict (only 2 slashes allowed)
    pub(super) fn take_date(&mut self) -> ParseResult<TokenMatch> {
        self.take_while(Token::Date, |ch| ch.is_ascii_digit() || ch == '/')
    }

    pub fn take_whitespace(&mut self) {
        let new_input = self.input.trim_start();
        self.move_input(self.input.len() - new_input.len());
    }

    fn match_token(&self, token: Token, str_match: &str) -> TokenMatch {
        TokenMatch {
            token,
            str_match: str_match.to_string(),
            position: self.position,
            cell_offset: self.cell_offset,
            source_code: self.runtime.source_code.clone(),
        }
    }

    pub(super) fn unknown_string(&self, message: &str) -> ParseError {
        UnknownToken {
            bad_input: self.input.to_string(),
            position: self.position,
            cell_offset: self.cell_offset,
            source_code: self.runtime.source_code.clone(),
        }
        .into_parse_error(message)
    }

    fn maybe_take(&mut self, token: Token, substring: &str) -> Option<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix(substring) {
            let token_match = self.match_token(token, substring);
            self.replace_input(without_match, substring.len());
            Some(token_match)
        } else {
            None
        }
    }

    fn maybe_take_regex(&mut self, tm: &TokenMatcher<Token>) -> Option<TokenMatch> {
        tm.try_match(self.input).map(|m| {
            let str_match = m.str_match;
            dbg!(&m);

            // move the offset past any observed whitespace, then build the token
            self.move_input(m.len_leading_whitespace);
            let token = self.match_token(tm.0, str_match);
            self.move_input(str_match.len());
            token
        })
    }

    fn take(&mut self, token: Token, substring: &str) -> ParseResult<TokenMatch> {
        self.take_whitespace();

        if let Some(without_match) = self.input.strip_prefix(substring) {
            let token_match = self.match_token(token, substring);
            self.replace_input(without_match, substring.len());
            Ok(token_match)
        } else {
            Err(self.unknown_string(&format!("Error parsing input, expected '{substring}'")))
        }
    }

    fn take_color(&mut self) -> ParseResult<TokenMatch> {
        let mut matched_alphas = 0;
        let mut saw_hash = false;
        let mut matched = "".to_string();

        self.take_whitespace();

        for c in self.input.chars() {
            if c == '#' && !saw_hash {
                saw_hash = true;
                matched.push(c);
            } else if c.is_alphanumeric() {
                if matched_alphas > 6 {
                    return Err(
                        self.unknown_string(&format!("Unexpected RGB color character: '{c}'"))
                    );
                }

                matched.push(c);
                matched_alphas += 1;
            } else {
                // either we're done or it's a syntax error
                if matched_alphas == 3 || matched_alphas == 6 {
                    break;
                }

                return Err(self
                    .unknown_string(&format!("Invalid character when parsing RGB color: '{c}'")));
            }
        }

        let token_match = self.match_token(Token::Color, &matched);
        self.move_input(matched.len());
        Ok(token_match)
    }

    fn take_string(&mut self) -> ParseResult<TokenMatch> {
        self.take_whitespace();

        if self.input.starts_with('\'') {
            Ok(self.take_single_quoted_string()?)
        } else {
            Ok(self.take_while(Token::String, |ch| ch.is_alphanumeric())?)
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    fn take_single_quoted_string(&mut self) -> ParseResult<TokenMatch> {
        let mut escape_mode = false;
        let mut matched = "".to_string();
        let mut start_quote = false;
        let mut end_quote = false;
        // TODO: pretty sure we can just use .enumerate() and get rid of the clippy allow above...
        // but I remember this code being tricky.  just make sure it's unit tested before removing
        let mut consumed = 0;

        self.take_whitespace();

        for c in self.input.chars() {
            // due to escaping rules, we don't always put what we consume on `matched`.  so we need
            // to keep track of it separately
            consumed += 1;

            if start_quote {
                if escape_mode {
                    matched.push(c);
                    escape_mode = false;
                } else if c == '\\' {
                    escape_mode = true;
                } else if c == '\'' {
                    end_quote = true;
                    break;
                } else {
                    matched.push(c);
                }
            } else if c == '\'' {
                start_quote = true;
            } else {
                return Err(self.unknown_string("Expected a starting single quote"));
            }
        }

        if start_quote && end_quote {
            let token_match = self.match_token(Token::String, &matched);
            self.move_input(consumed);
            Ok(token_match)
        } else {
            Err(self.unknown_string("Unterminated single-quoted string"))
        }
    }

    fn take_while<F>(&mut self, token: Token, while_fn: F) -> ParseResult<TokenMatch>
    where
        F: Fn(char) -> bool,
    {
        self.take_whitespace();

        let mut matched = "".to_string();

        for c in self.input.chars() {
            if while_fn(c) {
                matched.push(c);
            } else {
                break;
            }
        }

        if matched.is_empty() {
            Err(self.unknown_string(&format!("Expected a {token}")))
        } else {
            self.move_input(matched.len());
            Ok(self.match_token(token, &matched))
        }
    }

    fn move_input(&mut self, amount: CharOffset) {
        self.input = &self.input[amount..];
        self.cell_offset += amount;
    }

    fn replace_input(&mut self, new_input: &'a str, amount: CharOffset) {
        self.input = new_input;
        self.cell_offset += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::Runtime;

    fn test_lexer<'a>(lexer_input: &'a str, runtime: &'a Runtime) -> ModifierLexer<'a> {
        ModifierLexer::new(lexer_input, a1_notation::Address::new(0, 0), runtime)
    }

    #[test]
    fn maybe_take_comma() {
        let runtime = build_runtime();

        let mut lexer = test_lexer(" , more stuff", &runtime);
        assert_eq!(lexer.maybe_take_comma().unwrap().token, Token::Comma);
        assert_eq!(lexer.input, " more stuff");

        let mut lexer = test_lexer(" anything else", &runtime);
        assert!(lexer.maybe_take_comma().is_none());
        assert_eq!(lexer.input, "anything else");
    }

    #[test]
    fn maybe_take_date() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("  11/2/2024, 1, 2", &runtime);

        assert_eq!(lexer.maybe_take_date().unwrap().token, Token::Date);
        assert_eq!(lexer.input, ", 1, 2");
        assert_eq!(lexer.cell_offset, 11);
    }

    #[test]
    fn maybe_take_identifier() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("     foo bar baz", &runtime);

        assert_eq!(
            lexer.maybe_take_identifier().unwrap().token,
            Token::Identifier
        );
        assert_eq!(lexer.input, " bar baz");
        assert_eq!(lexer.cell_offset, 8);
    }

    #[test]
    fn maybe_take_single_quoted_string() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("     'foo bar' baz", &runtime);

        assert_eq!(
            lexer
                .maybe_take_single_quoted_string()
                .unwrap()
                .unwrap()
                .token,
            Token::String
        );
        assert_eq!(lexer.input, " baz");
        assert_eq!(lexer.cell_offset, 14);
    }

    #[test]
    fn maybe_take_start_modifier_modifier() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("[[", &runtime);

        assert_eq!(
            Token::StartCellModifier,
            lexer.maybe_take_start_modifier().unwrap().token
        );
    }

    #[test]
    fn maybe_take_start_modifier_row_modifier() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("![[", &runtime);

        assert_eq!(
            Token::StartRowModifier,
            lexer.maybe_take_start_modifier().unwrap().token
        );
    }

    #[test]
    fn maybe_take_start_modifier_none() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("foo", &runtime);

        assert!(lexer.maybe_take_start_modifier().is_none());
    }

    #[test]
    fn take_modifier_right_side() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("=foo_bar", &runtime);

        assert_eq!(
            "foo_bar",
            lexer.take_modifier_right_side().unwrap().str_match
        );
    }

    #[test]
    fn take_modifier_right_side_invalid() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("foo", &runtime);

        assert!(lexer.take_modifier_right_side().is_err());
    }

    #[test]
    fn maybe_take_equals() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("=", &runtime);

        assert!(lexer.maybe_take_equals().is_some());
    }

    #[test]
    fn maybe_take_slash() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("/", &runtime);

        assert!(lexer.maybe_take_slash().is_some());
    }

    #[test]
    fn take_token_color() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("#ABC123", &runtime);

        assert_eq!("#ABC123", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_color_shorthand() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("#ABC", &runtime);

        assert_eq!("#ABC", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_color_no_hash() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("ABC123", &runtime);

        assert_eq!("ABC123", lexer.take_token(Token::Color).unwrap().str_match);
    }

    #[test]
    fn take_token_end_modifier() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("]]", &runtime);

        assert_eq!(
            "]]",
            lexer.take_token(Token::EndModifier).unwrap().str_match
        );
    }

    #[test]
    fn take_token_equals() {
        let runtime = build_runtime();
        let mut lexer = test_lexer(" = ", &runtime);

        assert_eq!("=", lexer.take_token(Token::Equals).unwrap().str_match);
    }

    #[test]
    fn take_token_modifier_name() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("foo", &runtime);

        assert_eq!(
            "foo",
            lexer.take_token(Token::ModifierName).unwrap().str_match
        );
    }

    #[test]
    fn take_token_positive_number() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("15", &runtime);

        assert_eq!(
            "15",
            lexer.take_token(Token::PositiveNumber).unwrap().str_match
        );
    }

    #[test]
    fn take_token_string() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("string", &runtime);

        assert_eq!("string", lexer.take_token(Token::String).unwrap().str_match);
    }

    #[test]
    fn take_token_string_double_quoted() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("'this is \\' a quoted string\\''", &runtime);

        assert_eq!(
            "this is ' a quoted string'",
            lexer.take_token(Token::String).unwrap().str_match
        );
        // make sure it consumed `input` given the quoting rules
        assert_eq!("", lexer.input);
    }

    #[test]
    fn take_token_slash() {
        let runtime = build_runtime();
        let mut lexer = test_lexer(" / ", &runtime);

        assert_eq!("/", lexer.take_token(Token::Slash).unwrap().str_match);
    }

    #[test]
    fn take_token_invalid() {
        let runtime = build_runtime();
        let mut lexer = test_lexer("foo", &runtime);

        assert!(lexer.take_token(Token::PositiveNumber).is_err());
    }

    #[test]
    fn rest() {
        let runtime = build_runtime();
        let mut lexer = test_lexer(" / = rest", &runtime);

        lexer.take_token(Token::Slash).unwrap();
        lexer.take_token(Token::Equals).unwrap();

        assert_eq!("rest", lexer.rest());
    }
}
