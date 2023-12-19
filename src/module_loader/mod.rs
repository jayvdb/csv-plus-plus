//! # ModuleLoader
//!
//! A multithreaded module loader that will resursively load the dependencies for a given
//! `Scope`.
//!
// TODO:
// * make it so that `---` is not required
use crate::parser::code_section_parser::CodeSectionParser;
use crate::{
    compiler_error, ArcSourceCode, Error, Module, ModulePath, Result, Scope, SourceCode,
    Spreadsheet,
};
use log::{debug, info};
use petgraph::graphmap;
use std::collections;
use std::path;
use std::sync;
use std::thread;

mod dependency;
use dependency::{Dependency, DependencyRelation};

type ArcRwLock<T> = sync::Arc<sync::RwLock<T>>;

type LoadedModules = collections::HashMap<ModulePath, Dependency>;

type Attempted = ArcRwLock<collections::HashSet<ModulePath>>;
type Loaded = ArcRwLock<LoadedModules>;
type Failed = ArcRwLock<collections::HashMap<ModulePath, Error>>;

#[derive(Debug)]
pub(super) struct ModuleLoader<'a> {
    main_module: &'a Module,
    attempted: Attempted,
    loaded: Loaded,
    failed: Failed,
}

// TODO: ideally this shouldn't take a $source_code and the calling part does the map_err
macro_rules! eval_fns_or_vars {
    ($scope:ident, $functions_or_variables:ident, $source_code:expr) => {{
        for (name, ast) in $scope.$functions_or_variables.clone().into_iter() {
            $scope.$functions_or_variables.insert(
                name,
                ast.eval(&$scope, None)
                    .map_err(|e| $source_code.eval_error(e, None))?,
            );
        }
    }};
}

macro_rules! eval_scope {
    ($scope:ident, $source_code:expr) => {
        eval_fns_or_vars!($scope, variables, $source_code);
        eval_fns_or_vars!($scope, functions, $source_code);
    };
}

// TODO:
// * get rid of unwrap()s
// * see if I can reduce the clone()s
impl<'a> ModuleLoader<'a> {
    /// Recursively load the dependencies from the given `scope` while collecting any errors into
    /// `failed` and sucesses into `loaded`. The idea being that we want to show as many errors as
    /// possible to the user (otherwise it's annoying to have them fix and re-compile one-by-one),
    /// so we accumulate and keep going.  But in the end fail if there are any errors at all.
    pub(super) fn load_dependencies(module: &'a Module) -> Result<ModuleLoader<'a>> {
        let module_loader = Self {
            main_module: module,
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
        };
        module_loader.load(module, DependencyRelation::Direct)?;

        Ok(module_loader)
    }

    /// Returns only the direct dependencies for this module graph.  For example if our Module A
    /// requires Module B which in turn requires Module C, we will only get vars & functions from
    /// Module B, not from Module C (or any other indirect dependencies)
    pub(super) fn into_direct_dependencies(self) -> Result<Scope> {
        if self.has_failures() {
            let failed = sync::Arc::try_unwrap(self.failed).unwrap().into_inner()?;
            Err(Error::ModuleLoadErrors(failed))
        } else {
            self.direct_dependencies()
        }
    }

    /// Extract all direct dependencies on `scope`.  
    fn direct_dependencies(self) -> Result<Scope> {
        let loaded = sync::Arc::try_unwrap(self.loaded).unwrap().into_inner()?;
        info!("Resolving {} module dependencies", loaded.len());

        let mut dep_graph: graphmap::UnGraphMap<_, ()> = graphmap::UnGraphMap::new();

        dep_graph.add_node(&self.main_module.module_path);

        // load all of the direct dependencies
        for p in &self.main_module.required_modules {
            dep_graph.add_node(p);
            dep_graph.add_edge(&self.main_module.module_path, p, ());
        }

        for (p, dep) in loaded.iter() {
            dep_graph.add_node(p);

            for required_dep in &dep.module.required_modules {
                dep_graph.add_node(required_dep);
                dep_graph.add_edge(p, required_dep, ());
            }
        }

        debug!("Loaded dependency graph {:?}", &dep_graph);

        // now that we have a graph, use Tarjan's algo to give us a topological sort which will
        // represent the dependencies in the order they need to be resolved.
        let resolution_order = petgraph::algo::tarjan_scc(&dep_graph)
            .into_iter()
            .flatten()
            .filter_map(|p| loaded.get(p));

        debug!("Resolving dependencies in order {:?}", &resolution_order);

        let mut evaled = collections::HashMap::<&ModulePath, Scope>::new();

        for dep in resolution_order.into_iter() {
            let mut local_scope = dep.module.scope.clone();
            for req_path in dep.module.required_modules.iter().rev() {
                let dep_scope = 
                    // look in `evaled` first (let it take precedence)
                    if let Some(s) = evaled.get(req_path) {
                        s.clone()
                    // otherwise look in `loaded`
                    } else if let Some(dep) = loaded.get(req_path) {
                        dep.module.scope.clone()
                    } else {
                        compiler_error(format!(
                                "Expected module to have been loaded: {req_path}"
                        ))
                    };

                // merge the scopes together, but let ours take precedent. because if you
                // define a variable that has the same name as an import, the assumption is
                // you'll be shadowing it
                local_scope = dep_scope.merge(local_scope);
            }

            eval_scope!(local_scope, dep.module.source_code);

            evaled.insert(&dep.module.module_path, local_scope);
        }

        let mut resolved_scope = self.main_module.scope.clone();
        for req_path in self.main_module.required_modules.iter() {
            resolved_scope = evaled.remove(req_path).unwrap().merge(resolved_scope);
        }

        Ok(resolved_scope)
    }

    fn has_failures(&self) -> bool {
        !self.failed.try_read().unwrap().is_empty()
    }

    fn load(&self, module: &Module, dependency_relation: DependencyRelation) -> Result<()> {
        let mut to_attempt = vec![];
        // hold a lock while we reserve all of the dependencies we're going to resolve (by
        // preemptively marking them in `attempted`)
        {
            let mut attempted = self.attempted.write()?;
            for module_path in &module.required_modules {
                if attempted.contains(module_path) {
                    // another modules has already loaded it
                    continue;
                } else {
                    attempted.insert(module_path.clone());
                    to_attempt.push(module_path.clone());
                }
            }
        }

        // now a thread for each module to load, and they'll recurse back to this function if they
        // in turn have modules to load
        thread::scope(|s| {
            for module_path in to_attempt {
                s.spawn(|| self.load_module(module_path, dependency_relation));
            }
        });

        Ok(())
    }

    // TODO: can I use the `TryFrom<PathBuf> for Module`?
    fn load_module(
        &self,
        module_path: ModulePath,
        dependency_relation: DependencyRelation,
    ) -> Result<()> {
        let p: path::PathBuf = module_path.clone().into();

        // load the source code
        let source_code = match SourceCode::try_from(p) {
            Ok(s) => ArcSourceCode::new(s),
            Err(e) => {
                self.failed.write()?.insert(module_path, e);
                return Ok(());
            }
        };

        // parse the code section
        if let Some(scope_source) = &source_code.code_section {
            // TODO: this should use the csvpo cache if there is one
            match CodeSectionParser::parse(scope_source, source_code.clone()) {
                Ok((scope, required_modules)) => {
                    let mut loaded_module = Module::new(
                        source_code,
                        module_path.clone(),
                        scope,
                        Spreadsheet::default(),
                    );
                    loaded_module.required_modules = required_modules;

                    // recursively load the newly loaded code section's dependencies (which are
                    // transitive at this point)
                    self.load(&loaded_module, DependencyRelation::Transitive)?;

                    self.loaded.write()?.insert(
                        module_path,
                        Dependency {
                            relation: dependency_relation,
                            module: loaded_module,
                        },
                    );
                }
                Err(e) => {
                    self.failed.write()?.insert(module_path, e);
                }
            }
        } else {
            self.failed.write()?.insert(
                module_path.clone(),
                Error::ModuleLoadError(
                    "This module does not have a code section (but you imported it)".to_string(),
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::test_utils::*;
    use crate::*;
    use std::sync;

    #[test]
    fn load_dependencies_empty() {
        assert!(ModuleLoader::load_dependencies(&build_module()).is_ok());
    }

    #[test]
    fn load_dependencies_require_does_not_exist() {
        let mut module = build_module();
        module.required_modules.push(ModulePath::new("bar"));
        let module_loader = ModuleLoader::load_dependencies(&module).unwrap();

        assert_eq!(module_loader.failed.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.loaded.read().unwrap().len(), 0);
    }

    #[test]
    fn load_dependencies_valid_files() {
        let mod1 = TestFile::new(
            "csvpp",
            "
a := 42
---
        ",
        );
        let mod2 = TestFile::new(
            "csvpp",
            "
b := 24
---
        ",
        );
        let mod1_path: ModulePath = (&mod1).into();
        let mod2_path: ModulePath = (&mod2).into();
        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![mod1_path.clone(), mod2_path.clone()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module).unwrap();
        let loaded = module_loader.loaded.read().unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 2);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
        assert_eq!(
            loaded
                .get(&mod1_path)
                .unwrap()
                .module
                .scope
                .variables
                .get("a")
                .unwrap(),
            &Ast::new(Node::var("a", VariableValue::Ast(42.into()))),
        );
        assert_eq!(
            loaded
                .get(&mod2_path)
                .unwrap()
                .module
                .scope
                .variables
                .get("b")
                .unwrap(),
            &Ast::new(Node::var("b", VariableValue::Ast(24.into()))),
        );
    }

    #[test]
    fn load_in_directory() {
        let dep_mod = TestFile::new_in_dir(
            "csvpp",
            "
a := 42
---
        ",
        );
        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&dep_mod).into()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module).unwrap();

        assert_eq!(module_loader.loaded.read().unwrap().len(), 1);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 1);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    #[test]
    fn load_dependencies_double_load() {
        let mod1 = TestFile::new(
            "csvpp",
            "
a := 42
---
        ",
        );
        let mod1_path: ModulePath = (&mod1).into();
        let mod2 = TestFile::new(
            "csvpp",
            &format!(
                "
use {mod1_path}
b := 24
---
        "
            ),
        );

        let module = Module {
            module_path: ModulePath::new("main"),
            required_modules: vec![(&mod1).into(), (&mod2).into()],
            ..build_module()
        };
        let module_loader = ModuleLoader::load_dependencies(&module).unwrap();

        assert_eq!(module_loader.loaded.read().unwrap().len(), 2);
        assert_eq!(module_loader.attempted.read().unwrap().len(), 2);
        assert_eq!(module_loader.failed.read().unwrap().len(), 0);
    }

    #[test]
    fn into_direct_dependencies_empty() {
        let module_loader = ModuleLoader {
            main_module: &build_module(),
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
        };

        assert!(module_loader.into_direct_dependencies().is_ok());
    }

    #[test]
    fn into_direct_dependencies_error() {
        let module_loader = ModuleLoader {
            main_module: &build_module(),
            attempted: Default::default(),
            loaded: Default::default(),
            failed: Default::default(),
        };
        module_loader.failed.write().unwrap().insert(
            ModulePath::new("foo"),
            Error::InitError("failed".to_string()),
        );

        assert!(module_loader.into_direct_dependencies().is_err());
    }

    #[test]
    fn into_direct_dependencies_variable() {
        // main -> a -> b -> c
        let mut loaded = collections::HashMap::new();

        // var_from_a depends on var_from_b
        let mut mod_a = Module {
            module_path: ModulePath::new("a"),
            required_modules: vec![ModulePath::new("b")],
            ..build_module()
        };
        mod_a.scope.variables.insert(
            "var_from_a".to_string(),
            Ast::new(Node::reference("var_from_b")),
        );
        loaded.insert(ModulePath::new("a"), Dependency::direct(mod_a));

        // var_from_b depends on var_from_c
        let mut mod_b = Module {
            module_path: ModulePath::new("b"),
            required_modules: vec![ModulePath::new("c")],
            ..build_module()
        };
        mod_b.scope.variables.insert(
            "var_from_b".to_string(),
            Ast::new(Node::reference("var_from_c")),
        );
        loaded.insert(ModulePath::new("b"), Dependency::transitive(mod_b));

        // var_from_c resolves to 420
        let mut mod_c = Module {
            module_path: ModulePath::new("c"),
            required_modules: vec![],
            ..build_module()
        };
        mod_c
            .scope
            .variables
            .insert("var_from_c".to_string(), Ast::new(420.into()));
        loaded.insert(ModulePath::new("c"), Dependency::transitive(mod_c));

        let main_module = Module {
            module_path: ModulePath::new("foo"),
            required_modules: vec![ModulePath::new("a")],
            ..build_module()
        };
        let module_loader = ModuleLoader {
            main_module: &main_module,
            attempted: Default::default(),
            loaded: sync::Arc::new(sync::RwLock::new(loaded)),
            failed: Default::default(),
        };

        let dependencies = module_loader.into_direct_dependencies().unwrap();

        assert_eq!(
            dependencies.variables.get("var_from_a").unwrap(),
            &Ast::new(420.into())
        );
    }

    #[test]
    fn into_direct_dependencies_function() {
        // TODO
    }

    #[test]
    fn into_direct_dependencies_shadowing() {
        // TODO make sure it shadows variables as we'd expect
    }

    #[test]
    fn into_direct_dependencies_cyclic() {
        // TODO
    }

    #[test]
    fn direct_dependencies_cyclic() {
        // TODO
    }
}
