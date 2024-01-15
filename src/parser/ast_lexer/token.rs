#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Boolean,
    CloseParen,
    CodeSectionEof,
    Comma,
    Comment,
    DoubleQuotedString,
    Eof,
    Float,
    FunctionDefinition,
    InfixOperator,
    Integer,
    Newline,
    OpenParen,
    Reference,
    UseModule,
    VarAssign,
}
