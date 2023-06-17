//! # AST (abstract syntaX tree) Functions
//!
//! `Node` represents a building block of the parsed language, with a 
//!
use core::fmt::Debug;
use core::fmt::Display;
// use serde::{Serialize, Deserialize};
use std::any::Any;
use std::collections::HashMap;

mod boolean;
mod date_time;
mod float;
mod function;
mod function_call;
mod infix_function_call;
mod integer;
mod reference;
mod text;

pub use boolean::Boolean;
pub use date_time::DateTime;
pub use float::Float;
pub use function::Function;
pub use function_call::FunctionCall;
pub use infix_function_call::InfixFunctionCall;
pub use integer::Integer;
pub use reference::Reference;
pub use text::Text;

type NodeId = String;

type FunctionArgs = Vec<String>;
type FunctionName = String;

pub trait NodeWithId: Debug + Display {
    /// An `id` gives a function a `impl Node` a unique identifier.  For example a `Function`
    /// implements `NodeWithId` and `Reference` implements `id_ref()` in a way that can point to it
    fn id(&self) -> NodeId;
}

// TODO can I implement PartialEq and call my own eq function?
// TODO add Send + Sync?
// TODO add Serialize + Deserialize
pub trait Node: Debug + Display {
    // TODO not sure yet how evaluation will work
    // fn evaluate(position, variables) -> Cell;
    
    /// What allows one Node (a `FunctionCall` or a `Reference`) to point to another node.  Think
    /// about if a user is referencing a variable - in the AST this would appear as a `Reference`
    /// which has an `id_ref() -> Some(reference.name)`.
    fn id_ref(&self) -> Option<NodeId> {
        None
    }

    fn eq(&self, other: &dyn Any) -> bool;
}

impl PartialEq for Box<dyn Node> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}


// TODO move to AstParser?
pub fn from_key_value_args(_key_value_args: String) -> HashMap<String, Box<dyn Node>> {
    // TODO parse _key_value_args
    HashMap::new()
}