//! This module includes the definition of prefab library blocks
//! We'll use json, because it's easy

use crate::ffi::{InterconnectDirection, PinKind};
use crate::netlist::{ModuleIndex, Netlist};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryModule {
    name: String,
    inputs: Vec<LibraryPin>,
    outputs: Vec<LibraryPin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryPin {
    name: String,
    kind: PinKind,
}

impl Netlist {
    pub fn get_library_module(&self, idx: ModuleIndex) -> Option<LibraryModule> {
        let m = self.get_module(idx)?;
        let inputs = m
            .inputs()
            .filter_map(|x| {
                self.get_pin(*x).and_then(|x| {
                    Some(LibraryPin {
                        name: x.name().to_string(),
                        kind: x.kind(),
                    })
                })
            })
            .collect();
        let outputs = m
            .outputs()
            .filter_map(|x| {
                self.get_pin(*x).and_then(|x| {
                    Some(LibraryPin {
                        name: x.name().to_string(),
                        kind: x.kind(),
                    })
                })
            })
            .collect();
        Some(LibraryModule {
            name: m.name().to_string(),
            inputs,
            outputs,
        })
    }

    pub fn add_module_from_library(&mut self, module: LibraryModule) -> ModuleIndex {
        // Add the module
        let mi = self.add_module(module.name);
        // Add all the ports
        for port in module.inputs {
            self.add_pin(mi, port.name, port.kind, InterconnectDirection::Input);
        }
        for port in module.outputs {
            self.add_pin(mi, port.name, port.kind, InterconnectDirection::Output);
        }
        mi
    }

    pub fn add_module_from_json(&mut self, mod_json: &str) -> Result<ModuleIndex> {
        let module = serde_json::from_str(mod_json)?;
        Ok(self.add_module_from_library(module))
    }

    pub fn dump_module_to_json(&self, idx: ModuleIndex) -> Option<String> {
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
                LibraryPin {
                    name: "A".to_owned(),
                    kind: PinKind::Wire,
                },
                LibraryPin {
                    name: "B".to_owned(),
                    kind: PinKind::Wire,
                },
            ],
            outputs: vec![LibraryPin {
                name: "Out".to_owned(),
                kind: PinKind::Wire,
            }],
        };
        // Add it
        netlist.add_module_from_library(logical);
    }
}
