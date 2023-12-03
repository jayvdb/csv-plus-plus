use crate::ast::{Functions, Variables};
use crate::ModulePath;

mod display;

#[derive(Debug, Default)]
pub struct CodeSection {
    pub(crate) functions: Functions,
    pub(crate) required_modules: Vec<ModulePath>,
    pub(crate) variables: Variables,
}