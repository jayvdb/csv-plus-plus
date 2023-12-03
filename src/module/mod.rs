//! # Module
//!
//! A `module` holds the final compiled state for a single csv++ source file
//!
// TODO:
// * we need more unit tests around the various eval phases
//      - fills
//      - row vs cell variable definitions
// * eval cells in parallel (rayon)
// * make sure there is only one infinite fill in the docs (ones can follow it, but they have to
//      be finite and subtract from it
use crate::ast::{Functions, Variables};
use crate::{CodeSection, Compiler, ModulePath, Result, Spreadsheet};
use std::cell;
use std::cmp;
use std::fs;

mod display;
mod module_loader;

use module_loader::ModuleLoader;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Module {
    pub functions: Functions,
    pub module_path: ModulePath,
    pub spreadsheet: cell::RefCell<Spreadsheet>,
    pub variables: Variables,
    pub compiler_version: String,
}

impl Module {
    pub(crate) fn load_main(
        spreadsheet: Spreadsheet,
        code_section: CodeSection,
        module_path: ModulePath,
    ) -> Result<Self> {
        let spreadsheet_vars = spreadsheet.variables();

        let module_loader = ModuleLoader::default();
        module_loader.load(&code_section)?;
        let _loaded_modules = module_loader.into_modules_loaded()?;

        Ok(Self {
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            functions: code_section.functions,
            module_path,
            spreadsheet: cell::RefCell::new(spreadsheet),
            variables: code_section
                .variables
                .into_iter()
                .chain(spreadsheet_vars)
                .collect(),
        })
    }

    pub(crate) fn write_object_file(&self, compiler: &Compiler) -> Result<()> {
        if !compiler.options.use_cache {
            compiler.info("Not writing object file");
            return Ok(());
        }

        let object_code_filename = compiler.source_code.object_code_filename();

        compiler.progress("Writing object file");

        let object_file = fs::File::create(object_code_filename).map_err(|e| {
            compiler.error(format!("IO error: {e:?}"));
            compiler
                .source_code
                .object_code_error(format!("Error opening object code for writing: {e}"))
        })?;

        serde_cbor::to_writer(object_file, self).map_err(|e| {
            compiler.error(format!("CBOR write error: {e:?}"));
            compiler
                .source_code
                .object_code_error(format!("Error serializing object code for writing: {e}"))
        })?;

        Ok(())
    }

    pub(crate) fn read_from_object_file(compiler: &Compiler) -> Result<Option<Self>> {
        if !compiler.options.use_cache {
            compiler.info("Not reading object file");
            return Ok(None);
        }

        let sc = &compiler.source_code;
        let obj_file = sc.object_code_filename();

        // does the object code file even exist?
        if !obj_file.exists() {
            compiler.info("Attempted to read object file but it does not exist");
            return Ok(None);
        }

        let obj_file_modified = fs::metadata(&obj_file)
            .and_then(|s| s.modified())
            .map_err(|e| sc.object_code_error(format!("Unable to stat object code: {e}")))?;
        let source_file_modified = fs::metadata(&compiler.source_code.filename)
            .and_then(|s| s.modified())
            .map_err(|e| sc.object_code_error(format!("Unable to stat source code: {e}")))?;

        // is the object code more recent than the source? (i.e., no changes since it was last
        // written)
        if source_file_modified > obj_file_modified {
            return Ok(None);
        }

        let obj_file_reader = fs::File::open(&obj_file)
            .map_err(|e| sc.object_code_error(format!("Error opening object code: {e}")))?;

        let Ok(loaded_module): std::result::Result<Self, serde_cbor::Error> =
            serde_cbor::from_reader(obj_file_reader)
        else {
            // if we fail to load the old object file just warn about it and move on.  for whatever
            // reason (written by an old version) it's not compatible with our current version
            compiler.warn(format!(
                "Error loading object code from {}.  Was it written with an old version of csv++?",
                obj_file.display()
            ));
            return Ok(None);
        };

        let current_version = env!("CARGO_PKG_VERSION").to_string();
        let this_version = semver::Version::parse(&current_version).map_err(|e| {
            sc.object_code_error(format!("Unable to parse version `{current_version}`: {e}"))
        })?;
        let loaded_version =
            semver::Version::parse(&loaded_module.compiler_version).map_err(|e| {
                sc.object_code_error(format!(
                    "Unable to parse loaded module version `{}`: {e}",
                    &loaded_module.compiler_version
                ))
            })?;

        // if the version is less than ours, don't use it and recompile instead.  otherwise we can
        // trust that it's ok to use
        Ok(match this_version.cmp(&loaded_version) {
            cmp::Ordering::Equal | cmp::Ordering::Greater => Some(loaded_module),
            cmp::Ordering::Less => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::*;
    use std::collections;

    #[test]
    fn load_main_with_code_section() {
        let mut functions = collections::HashMap::new();
        functions.insert("foo".to_string(), Ast::new(1.into()));
        let mut variables = collections::HashMap::new();
        variables.insert("bar".to_string(), Ast::new(2.into()));
        let code_section = CodeSection {
            functions,
            variables,
            ..Default::default()
        };
        let module = Module::load_main(
            Spreadsheet::default(),
            code_section,
            ModulePath(vec!["foo".to_string()]),
        )
        .unwrap();

        assert!(module.functions.contains_key("foo"));
        assert!(module.variables.contains_key("bar"));
    }

    #[test]
    fn load_main_without_code_section() {
        let module = Module::load_main(
            Spreadsheet::default(),
            CodeSection::default(),
            ModulePath(vec!["foo".to_string()]),
        )
        .unwrap();

        assert!(module.functions.is_empty());
        assert!(module.variables.is_empty());
    }
}