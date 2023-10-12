//! # VerticalAlign
//!
use crate::error::ModifierParseError;
use crate::parser::modifier_lexer::TokenMatch;
use serde::{Deserialize, Serialize};

/// The possible values for aligning a cell vertically.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum VerticalAlign {
    Bottom,
    Center,
    Top,
}

impl TryFrom<TokenMatch> for VerticalAlign {
    type Error = ModifierParseError;

    fn try_from(input: TokenMatch) -> Result<Self, Self::Error> {
        match input.str_match.to_lowercase().as_str() {
            "b" | "bottom" => Ok(Self::Bottom),
            "c" | "center" => Ok(Self::Center),
            "t" | "top" => Ok(Self::Top),
            _ => Err(ModifierParseError::new(
                "valign",
                input,
                Some(&["bottom (b)", "center (c)", "top (t)"]),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn try_from_top() {
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_modifier_token_match("t")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_modifier_token_match("top")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Top,
            VerticalAlign::try_from(build_modifier_token_match("TOP")).unwrap()
        );
    }

    #[test]
    fn try_from_center() {
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_modifier_token_match("c")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_modifier_token_match("center")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Center,
            VerticalAlign::try_from(build_modifier_token_match("CENTER")).unwrap()
        );
    }

    #[test]
    fn try_from_bottom() {
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_modifier_token_match("b")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_modifier_token_match("bottom")).unwrap()
        );
        assert_eq!(
            VerticalAlign::Bottom,
            VerticalAlign::try_from(build_modifier_token_match("BOTTOM")).unwrap()
        );
    }
}
