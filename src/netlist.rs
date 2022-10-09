//! This module defines the internal graph structure of the netlist

use crate::ffi::{InterconnectDirection, PinKind};
use anyhow::bail;
use generational_arena::{Arena, Index};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PinIndex(pub Index);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModuleIndex(pub Index);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WireIndex(pub Index);

impl Display for PinIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Index:{}, Generation:{}",
            self.0.into_raw_parts().0,
            self.0.into_raw_parts().1
        )
    }
}

impl Display for ModuleIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Index:{}, Generation:{}",
            self.0.into_raw_parts().0,
            self.0.into_raw_parts().1
        )
    }
}

impl Display for WireIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Index:{}, Generation:{}",
            self.0.into_raw_parts().0,
            self.0.into_raw_parts().1
        )
    }
}

#[derive(Debug)]
enum Interconnect {
    Input { connection: Option<PinIndex> },
    Output { connections: Vec<PinIndex> },
}

impl Interconnect {
    pub fn direction(&self) -> InterconnectDirection {
        match self {
            Interconnect::Input { .. } => InterconnectDirection::Input,
            Interconnect::Output { .. } => InterconnectDirection::Output,
        }
    }

    pub fn is_input(&self) -> bool {
        self.direction() == InterconnectDirection::Input
    }

    pub fn is_output(&self) -> bool {
        self.direction() == InterconnectDirection::Output
    }
}

impl Display for PinKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                PinKind::Wire => "wire",
                PinKind::Integer => "integer",
                PinKind::Real => "real",
                _ => unreachable!(),
            }
        )
    }
}

impl PinKind {
    /// Check to see if this pin kind is compatible with another
    pub fn compatible(&self, other: PinKind) -> bool {
        // Right now, just check equality. Eventually we want to use some casting rules
        *self == other
    }
}

#[derive(Debug)]
pub struct Pin {
    name: String,
    kind: PinKind,
    interconnect: Interconnect,
    parent: ModuleIndex,
}

impl Pin {
    pub fn new(
        name: String,
        kind: PinKind,
        direction: InterconnectDirection,
        parent: ModuleIndex,
    ) -> Self {
        Self {
            name,
            kind,
            parent,
            interconnect: match direction {
                InterconnectDirection::Input => Interconnect::Input { connection: None },
                InterconnectDirection::Output => Interconnect::Output {
                    connections: vec![],
                },
                _ => unreachable!(),
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn direction(&self) -> InterconnectDirection {
        self.interconnect.direction()
    }

    pub fn is_input(&self) -> bool {
        self.interconnect.is_input()
    }

    pub fn is_output(&self) -> bool {
        self.interconnect.is_output()
    }

    pub fn kind(&self) -> PinKind {
        self.kind
    }
}

#[derive(Debug, PartialEq)]
pub struct Module {
    name: String,
    inputs: Vec<PinIndex>,
    outputs: Vec<PinIndex>,
    // Globally unique module id
    id: i32,
}

impl Module {
    pub fn new(name: String, id: i32) -> Self {
        Self {
            name,
            inputs: vec![],
            outputs: vec![],
            id,
        }
    }

    /// Get the name of the module
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get an iterator over the inputs of the module
    pub fn inputs(&self) -> std::slice::Iter<PinIndex> {
        self.inputs.iter()
    }

    /// Get an iterator over the outputs of the module
    pub fn outputs(&self) -> std::slice::Iter<PinIndex> {
        self.outputs.iter()
    }

    /// Get the module id
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug)]
pub struct Netlist {
    modules: Arena<Module>,
    pins: Arena<Pin>,
    wires: Arena<(PinIndex, PinIndex)>,
    next_mod_idx: i32,
}

impl Default for Netlist {
    fn default() -> Self {
        Self::new()
    }
}

impl Netlist {
    pub fn new() -> Self {
        Self {
            modules: Arena::new(),
            pins: Arena::new(),
            wires: Arena::new(),
            next_mod_idx: 0,
        }
    }

    /// Get an iterator over the modules
    pub fn modules(&self) -> generational_arena::Iter<Module> {
        self.modules.iter()
    }

    /// Get an iterator over the pins
    pub fn pins(&self) -> generational_arena::Iter<Pin> {
        self.pins.iter()
    }

    /// Get an iterator over the wires
    pub fn wires(&self) -> generational_arena::Iter<(PinIndex, PinIndex)> {
        self.wires.iter()
    }

    /// Get a particular module by index
    pub fn get_module(&self, idx: ModuleIndex) -> Option<&Module> {
        self.modules.get(idx.0)
    }

    /// Get a particular pin by index
    pub fn get_pin(&self, idx: PinIndex) -> Option<&Pin> {
        self.pins.get(idx.0)
    }

    /// Get a particular wire by index
    pub fn get_wire(&self, idx: WireIndex) -> Option<&(PinIndex, PinIndex)> {
        self.wires.get(idx.0)
    }

    /// Add a pin to the netlist, associating it with a module by module index `idx`.
    /// Returns `None` if no module exists in the given index.
    pub fn add_pin(
        &mut self,
        idx: ModuleIndex,
        name: String,
        kind: PinKind,
        direction: InterconnectDirection,
    ) -> Option<PinIndex> {
        // Grab the module and bail if it doesn't exist
        let m = self.modules.get_mut(idx.0)?;
        // If it does, insert the pin
        let pi = PinIndex(self.pins.insert(Pin::new(name, kind, direction, idx)));
        // And associate it with the module
        match direction {
            InterconnectDirection::Input => m.inputs.push(pi),
            InterconnectDirection::Output => m.outputs.push(pi),
            _ => unreachable!(),
        };
        // And return the index
        Some(pi)
    }

    /// Remove a pin by its pin index `idx`
    /// Returns `None` if no pin exists in the given index
    pub fn remove_pin(&mut self, idx: PinIndex) -> Option<()> {
        // Remove the pin (if it exists)
        let p = self.pins.remove(idx.0)?;
        // Disassociate this pin with its parent module if it exists
        if let Some(parent) = self.modules.get_mut(p.parent.0) {
            match p.direction() {
                InterconnectDirection::Input => parent.inputs.retain(|x| *x != idx),
                InterconnectDirection::Output => parent.outputs.retain(|x| *x != idx),
                _ => unreachable!(),
            }
        }
        // Unlink the pin's interconnnect
        match p.interconnect {
            // If this pin is an input and has a connection, it exists in the associated output pin's connection list (if that pin exists)
            Interconnect::Input { connection } => {
                if let Some(pi) = connection {
                    if let Some(p) = self.pins.get_mut(pi.0) {
                        match &mut p.interconnect {
                            Interconnect::Output { connections } => {
                                connections.retain(|x| *x != pi)
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            // If this pin is an output, it could be potentially driving several inputs.
            // If those inputs pins exist, make sure to remove this pin (the one we removed) from it's connection
            Interconnect::Output { connections } => {
                for pi in connections {
                    if let Some(p) = self.pins.get_mut(pi.0) {
                        match &mut p.interconnect {
                            Interconnect::Input { connection } => *connection = None,
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        // Finally, remove all the wires that invlove this pin
        self.wires.retain(|_, (x, y)| (*x != idx) && (*y != idx));
        // We're done!
        Some(())
    }

    /// Add a module to the netlist, returning the module index
    pub fn add_module(&mut self, name: String) -> ModuleIndex {
        // Grab the next index we can assign
        let id = self.next_mod_idx;
        // Increment the counter
        self.next_mod_idx += 1;
        // Add the module to the arena and return the index
        ModuleIndex(self.modules.insert(Module::new(name, id)))
    }

    /// Remove a module by it's module index `idx`, returning None if no such module exists
    pub fn remove_module(&mut self, idx: ModuleIndex) -> Option<()> {
        // Remove the module
        let m = self.modules.remove(idx.0)?;
        // Remove every pin associated with the module
        m.inputs.iter().for_each(|x| {
            self.remove_pin(*x);
        });
        m.outputs.iter().for_each(|x| {
            self.remove_pin(*x);
        });
        // We're done!
        Some(())
    }

    /// Try to form a connection between pins referenced by `a_idx` and `b_idx`.
    /// Will error if either pin doesn't exist or there is a problem when trying to form the connection.
    pub fn add_wire(&mut self, a_idx: PinIndex, b_idx: PinIndex) -> anyhow::Result<WireIndex> {
        // Check to make sure the pins are unique
        if a_idx == b_idx {
            bail!(ConnectionError::IdenticalPins);
        }
        // First get those pins by reference
        let (a, b) = self.pins.get2_mut(a_idx.0, b_idx.0);
        let a = a.ok_or(ConnectionError::BadIndex(a_idx))?;
        let b = b.ok_or(ConnectionError::BadIndex(b_idx))?;

        // Make sure the types are compatible
        if !(a.kind.compatible(b.kind)) {
            bail!(ConnectionError::Compatibility(a.kind, b.kind));
        }
        // Ensure we have an input and an output
        let ((input, input_idx), (output, output_idx)) = if a.is_input() && b.is_output() {
            ((a, a_idx), (b, b_idx))
        } else if b.is_input() && a.is_output() {
            ((b, b_idx), (a, a_idx))
        } else {
            bail!(ConnectionError::Direction);
        };
        // Make sure the input pin isn't already driven
        // If it is, error, if not, add the interconnects
        match &mut input.interconnect {
            Interconnect::Input { connection } => {
                if connection.is_some() {
                    bail!(ConnectionError::InputDriven);
                } else {
                    *connection = Some(output_idx);
                    match &mut output.interconnect {
                        Interconnect::Output { connections } => connections.push(input_idx),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
        // Finally, add the wire to the list of wires
        Ok(WireIndex(self.wires.insert((input_idx, output_idx))))
    }

    /// Remove a wire given its wire index, returning None if the index points to nothing
    pub fn remove_wire(&mut self, idx: WireIndex) -> Option<()> {
        // Get the pin indices from the wire
        let (a_idx, b_idx) = self.wires.remove(idx.0)?;
        // Then get those pins
        let (a, b) = self.pins.get2_mut(a_idx.0, b_idx.0);
        let a = a.expect("Wires should get deleted before pins do");
        let b = b.expect("Wires should get deleted before pins do");
        // Disambiguate input and output
        let ((input, input_idx), (output, _)) = if a.is_input() && b.is_output() {
            ((a, a_idx), (b, b_idx))
        } else {
            ((b, b_idx), (a, a_idx))
        };
        // Finally, cleanup the interconnects
        // Remove the input index from the list of connections in the output
        // And set the input connection to none
        match &mut input.interconnect {
            Interconnect::Input { connection } => {
                *connection = None;
                match &mut output.interconnect {
                    Interconnect::Output { connections } => connections.retain(|x| *x != input_idx),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
        // All done
        Some(())
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConnectionError {
    #[error("The supplied pin index `{0}` points to a pin that doesn't exist")]
    BadIndex(PinIndex),
    #[error("Pins a and b are identical")]
    IdenticalPins,
    #[error("The pins on either side of the connection are incompatible: {0} and {1}")]
    Compatibility(PinKind, PinKind),
    #[error("A wire must connect an input to an output")]
    Direction,
    #[error("The input is already driven. Remove the existing connection first.")]
    InputDriven,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_module() {
        let mut netlist = Netlist::new();
        let mi = netlist.add_module("my_name".to_owned());
        assert_eq!(netlist.get_module(mi).unwrap().name(), "my_name");
    }

    #[test]
    fn test_add_pin() {
        let mut netlist = Netlist::new();
        let mi = netlist.add_module("my_name".to_owned());
        let _ = netlist
            .add_pin(
                mi,
                "A".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let _ = netlist
            .add_pin(
                mi,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let _ = netlist
            .add_pin(
                mi,
                "C".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();
    }

    #[test]
    fn test_remove_pin() {
        let mut netlist = Netlist::new();
        let mi = netlist.add_module("my_name".to_owned());
        let pi_a = netlist
            .add_pin(
                mi,
                "A".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let pi_b = netlist
            .add_pin(
                mi,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        assert!(netlist.remove_pin(pi_a).is_some());
        assert_eq!(netlist.pins().len(), 1);
        assert_eq!(
            *netlist.get_module(mi).unwrap().inputs().next().unwrap(),
            pi_b
        );
    }

    #[test]
    fn test_remove_module() {
        let mut netlist = Netlist::new();
        let mi = netlist.add_module("my_name".to_owned());
        let mi_b = netlist.add_module("my_name_B".to_owned());
        let _ = netlist
            .add_pin(
                mi,
                "A".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let _ = netlist
            .add_pin(
                mi,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        netlist.remove_module(mi).unwrap();
        assert_eq!(netlist.modules().len(), 1);
        let m = netlist.modules().next().unwrap().0;
        assert_eq!(m, mi_b.0);
    }

    #[test]
    fn test_add_wire() {
        let mut netlist = Netlist::new();
        let m_1 = netlist.add_module("mod".to_owned());
        let a_1 = netlist
            .add_pin(
                m_1,
                "A".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let b_1 = netlist
            .add_pin(
                m_1,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let c_1 = netlist
            .add_pin(
                m_1,
                "C".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();

        let m_2 = netlist.add_module("mod2".to_owned());
        let a_2 = netlist
            .add_pin(
                m_2,
                "A".to_owned(),
                PinKind::Real,
                InterconnectDirection::Input,
            )
            .unwrap();
        let b_2 = netlist
            .add_pin(
                m_2,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let c_2 = netlist
            .add_pin(
                m_2,
                "C".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();

        // Bad connections
        netlist.add_wire(a_1, a_1).expect_err("Identical");
        netlist.add_wire(a_1, b_1).expect_err("Both inputs");
        netlist.add_wire(c_1, a_2).expect_err("Incompatible");

        // Good connection
        let wi = netlist.add_wire(c_1, b_2).unwrap();

        // Bad connection
        netlist.add_wire(c_2, b_2).expect_err("Input driven");

        // Make sure everything got hooked up
        assert_eq!(netlist.wires().len(), 1);
        assert_eq!(netlist.wires().next().unwrap().0, wi.0);
        let (input, output) = netlist.get_wire(wi).unwrap();
        let input_pin = netlist.get_pin(*input).unwrap();
        let output_pin = netlist.get_pin(*output).unwrap();

        if let Interconnect::Input { connection } = &input_pin.interconnect {
            if let Interconnect::Output { connections } = &output_pin.interconnect {
                assert_eq!(*connection, Some(*output));
                assert!(connections.contains(input));
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    #[test]
    fn test_remove_wire() {
        let mut netlist = Netlist::new();
        let m_1 = netlist.add_module("mod".to_owned());
        let c_1 = netlist
            .add_pin(
                m_1,
                "C".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();

        let m_2 = netlist.add_module("mod2".to_owned());
        let a_2 = netlist
            .add_pin(
                m_2,
                "A".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let b_2 = netlist
            .add_pin(
                m_2,
                "B".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();

        // Drive all the inputs of m_2 with the output of m_1
        let wi_1 = netlist.add_wire(c_1, a_2).unwrap();
        let _ = netlist.add_wire(c_1, b_2).unwrap();

        // Now remove wi_1
        assert!(netlist.remove_wire(wi_1).is_some());

        // And check that everything was cleaned up
        assert_eq!(netlist.wires().len(), 1);
        match &netlist.get_pin(c_1).unwrap().interconnect {
            Interconnect::Output { connections } => {
                assert_eq!(connections.len(), 1);
                assert_eq!(connections[0], b_2);
            }
            _ => unreachable!(),
        }
        match &netlist.get_pin(a_2).unwrap().interconnect {
            Interconnect::Input { connection } => {
                assert!(connection.is_none());
            }
            _ => unreachable!(),
        }
        match &netlist.get_pin(b_2).unwrap().interconnect {
            Interconnect::Input { connection } => {
                assert_eq!(connection.unwrap(), c_1);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_delete_mod_with_wires() {
        let mut netlist = Netlist::new();
        let m_1 = netlist.add_module("".to_owned());
        let out_1 = netlist
            .add_pin(
                m_1,
                "".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();

        let m_2 = netlist.add_module("".to_owned());
        let out_2 = netlist
            .add_pin(
                m_2,
                "".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Output,
            )
            .unwrap();

        let m_3 = netlist.add_module("".to_owned());
        let a = netlist
            .add_pin(
                m_3,
                "".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();
        let b = netlist
            .add_pin(
                m_3,
                "".to_owned(),
                PinKind::Wire,
                InterconnectDirection::Input,
            )
            .unwrap();

        let _ = netlist.add_wire(out_1, a).unwrap();
        let _ = netlist.add_wire(out_2, b).unwrap();

        assert_eq!(netlist.wires().len(), 2);
        assert_eq!(netlist.modules().len(), 3);
        assert_eq!(netlist.pins().len(), 4);
        // Remove mod 1
        assert!(netlist.remove_module(m_1).is_some());
        assert_eq!(netlist.wires().len(), 1);
        assert_eq!(netlist.modules().len(), 2);
        assert_eq!(netlist.pins().len(), 3);
        match netlist.get_pin(a).unwrap().interconnect {
            Interconnect::Input { connection } => {
                assert!(connection.is_none())
            }
            _ => unreachable!(),
        }
        match netlist.get_pin(b).unwrap().interconnect {
            Interconnect::Input { connection } => {
                assert_eq!(connection.unwrap(), out_2)
            }
            _ => unreachable!(),
        }
    }
}
