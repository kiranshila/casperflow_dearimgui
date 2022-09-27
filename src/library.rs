//! This module includes the definition of prefab library blocks
//! We'll use json, because it's easy

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::verilog::{Module, ModuleIndex, Netlist, Port, VerilogKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryModule {
    name: String,
    inputs: Vec<LibraryPort>,
    outputs: Vec<LibraryPort>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryPort {
    name: String,
    kind: VerilogKind,
}

impl Port {
    pub fn to_library(&self) -> LibraryPort {
        match self {
            Port::Input { name, kind, .. } => LibraryPort {
                name: name.clone(),
                kind: *kind,
            },
            Port::Output { name, kind, .. } => LibraryPort {
                name: name.clone(),
                kind: *kind,
            },
        }
    }
    pub fn from_library_input(port: LibraryPort) -> Self {
        Self::input(port.name, port.kind)
    }
    pub fn from_library_output(port: LibraryPort) -> Self {
        Self::output(port.name, port.kind)
    }
}

impl Netlist {
    pub fn get_library_module(&self, idx: ModuleIndex) -> Option<LibraryModule> {
        let m = self.get_module(idx)?;
        let inputs = m
            .inputs
            .iter()
            .filter_map(|x| self.get_port(*x).and_then(|x| Some(x.to_library())))
            .collect();
        let outputs = m
            .outputs
            .iter()
            .filter_map(|x| self.get_port(*x).and_then(|x| Some(x.to_library())))
            .collect();
        Some(LibraryModule {
            name: m.name.clone(),
            inputs,
            outputs,
        })
    }

    pub fn add_library_module(&mut self, module: LibraryModule) -> ModuleIndex {
        // Add the module
        let mi = self.add_module(Module::new(module.name));
        // Add all the ports
        for port in module.inputs {
            self.add_port(Port::from_library_input(port), mi);
        }
        for port in module.outputs {
            self.add_port(Port::from_library_output(port), mi);
        }

        mi
    }

    pub fn add_library_module_from_json(&mut self, mod_json: &str) -> Result<ModuleIndex> {
        let module = serde_json::from_str(mod_json)?;
        Ok(self.add_library_module(module))
    }

    pub fn dump_library_module_to_json(&self, idx: ModuleIndex) -> Option<String> {
        let module = self.get_library_module(idx)?;
        Some(serde_json::to_string_pretty(&module).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_library() {
        let mut netlist = Netlist::new();
        // Create a new library object
        let logical = LibraryModule {
            name: "Logical".to_owned(),
            inputs: vec![
                LibraryPort {
                    name: "A".to_owned(),
                    kind: Default::default(),
                },
                LibraryPort {
                    name: "B".to_owned(),
                    kind: Default::default(),
                },
            ],
            outputs: vec![LibraryPort {
                name: "Out".to_owned(),
                kind: Default::default(),
            }],
        };
        // Add it
        netlist.add_library_module(logical);
    }
}
