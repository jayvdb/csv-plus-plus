// TODO:
// * use clippy
// * move some of this into lib.rs rather than main.rs
// * use (read from) the object file if it exists
mod ast;
mod compiler;
mod cli_args;
mod error;
mod init;
mod modifier;
mod options;
mod output_target;
mod position;
mod rgb;
mod runtime;
mod source_code;
mod target;

pub use ast::*;
pub use cli_args::CliArgs;
pub use compiler::Cell;
pub use compiler::template::{Spreadsheet, Template};
pub use compiler::token_library::TokenLibrary;
pub use error::*;
pub use init::Init;
pub use modifier::Modifier;
pub use options::Options;
pub use output_target::OutputTarget;
pub use position::Position;
pub use rgb::Rgb;
pub use runtime::Runtime;
pub use source_code::SourceCode;
pub use target::CompilerTarget;

pub type Result<T> = std::result::Result<T, Error>;
