//! A Project contains several netlists, each netlist acting as a "submodule" or "subgraph"

use crate::{ netlist::{Netlist, Module, ModuleIndex}};

#[derive(Debug)]
pub struct Subgraph<'a>{
    inputs: Vec<ModuleIndex>,
    outputs: Vec<ModuleIndex>,
    netlist: &'a Netlist,
    name: String,
}

impl<'a> Subgraph<'a> {
    pub fn new(inputs: Vec<ModuleIndex>, outputs: Vec<ModuleIndex>, netlist: &'a Netlist) -> Self {
        // From the pints in `netlist`, we need to specify a few as inputs and ouptuts.
        // These are special nodes in that the won't have any parameters, just a single pin with
        // a type. We'll check that here.
        Self { inputs: todo!(), outputs: todo!(), netlist: todo!(), name: todo!() }
    }
}

// A subgraph input module has a single output pin
fn is_input_module(m: Module) -> bool {
    m.inputs().len() == 0 && m.outputs().len() == 1
}

// A subgraph output module has a single input pin
fn is_output_module(m: Module) -> bool {
    m.inputs().len() == 1 && m.outputs().len() == 0
}