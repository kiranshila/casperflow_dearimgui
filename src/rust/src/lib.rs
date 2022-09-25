pub mod verilog;

use std::sync::Mutex;

use generational_arena::Arena;
use lazy_static::lazy_static;
use verilog::{Module, ModuleIndex, Port, WireIndex};

lazy_static! {
    pub static ref NETLIST: Mutex<Netlist> = Mutex::new(Netlist {
        modules: Arena::new(),
        wires: vec![],
    });
}

#[derive(Debug)]
pub struct Netlist {
    modules: Arena<Module>,
    wires: Vec<(WireIndex, WireIndex)>,
}

#[repr(C)]
#[derive(Debug)]
pub enum ConnectionError {
    BadIndex,
    DirectionMismatch,
    TypeMismatch,
    InputDriven,
}

impl Netlist {
    pub fn new() -> Self {
        Self {
            modules: Arena::new(),
            wires: vec![],
        }
    }
    pub fn add_module(&mut self, module: Module) -> ModuleIndex {
        ModuleIndex(self.modules.insert(module))
    }
    pub fn connect(
        &mut self,
        output: WireIndex,
        input: WireIndex,
    ) -> Result<usize, ConnectionError> {
        // Check that input and output are different
        if input == output {
            return Err(ConnectionError::BadIndex);
        }
        let (in_port, out_port) = self.modules.get2_mut(input.0 .0, output.0 .0);

        // Check input exists and is input
        let in_port = in_port.and_then(|x| x.ports.get_mut(input.1 .0));
        let in_port = match in_port {
            Some(port) if matches!(port, Port::Input { .. }) => port,
            Some(_) => return Err(ConnectionError::DirectionMismatch),
            None => return Err(ConnectionError::BadIndex),
        };

        // Check output exists and is output
        let out_port = out_port.and_then(|x| x.ports.get_mut(output.1 .0));
        let out_port = match out_port {
            Some(port) if matches!(port, Port::Output { .. }) => port,
            Some(_) => return Err(ConnectionError::DirectionMismatch),
            None => return Err(ConnectionError::BadIndex),
        };

        // Check to make sure the types match
        if in_port.kind() != out_port.kind() {
            return Err(ConnectionError::TypeMismatch);
        }

        // Check to verify inputs have only one connection
        if matches!(
            in_port,
            Port::Input {
                connection: Some(_),
                ..
            }
        ) {
            return Err(ConnectionError::InputDriven);
        }
        // Drive the input
        if let Port::Input {
            ref mut connection, ..
        } = in_port
        {
            *connection = Some(output);
        }
        // Add to the list of driving inputs to the output
        if let Port::Output {
            ref mut connections,
            ..
        } = out_port
        {
            connections.push(input);
        }
        // Add the wire
        self.wires.push((input, output));
        // Return the index we just added
        Ok(self.wires.len() - 1)
    }
}

#[cfg(test)]
mod tests {

    use crate::verilog::{Port, VerilogKind};

    use super::*;

    #[test]
    fn test_netlists() {
        // Some Modules
        let mut and_1 = Module::new("and".to_owned());
        let and_a_idx = and_1.add_port(Port::input("A".to_owned(), VerilogKind::Logic(1)));
        let and_b_idx = and_1.add_port(Port::input("B".to_owned(), VerilogKind::Logic(1)));
        let and_out_idx = and_1.add_port(Port::output("Out".to_owned(), VerilogKind::Logic(1)));

        let and_2 = and_1.clone();

        let mut or = Module::new("or".to_owned());
        let or_a_idx = or.add_port(Port::input("A".to_owned(), VerilogKind::Logic(1)));
        let or_b_idx = or.add_port(Port::input("B".to_owned(), VerilogKind::Logic(1)));
        let or_out_idx = or.add_port(Port::output("Out".to_owned(), VerilogKind::Logic(1)));

        // Some "external" ports
        let mut in_1 = Module::new("1".to_owned());
        let in_1_in_idx = in_1.add_port(Port::output("In".to_owned(), VerilogKind::Logic(1)));
        let mut in_2 = Module::new("2".to_owned());
        let in_2_in_idx = in_2.add_port(Port::output("In".to_owned(), VerilogKind::Logic(1)));
        let mut in_3 = Module::new("3".to_owned());
        let in_3_in_idx = in_3.add_port(Port::output("In".to_owned(), VerilogKind::Logic(1)));
        let mut in_4 = Module::new("4".to_owned());
        let in_4_in_idx = in_4.add_port(Port::output("In".to_owned(), VerilogKind::Logic(1)));
        let mut out = Module::new("5".to_owned());
        let out_out_idx = out.add_port(Port::input("Out".to_owned(), VerilogKind::Logic(1)));

        // Add modules to netlist
        let mut netlist = Netlist::new();
        let and_1_idx = netlist.add_module(and_1);
        let and_2_idx = netlist.add_module(and_2);
        let or_idx = netlist.add_module(or);
        let in_1_idx = netlist.add_module(in_1);
        let in_2_idx = netlist.add_module(in_2);
        let in_3_idx = netlist.add_module(in_3);
        let in_4_idx = netlist.add_module(in_4);
        let out_idx = netlist.add_module(out);

        // Make connections
        netlist
            .connect((in_1_idx, in_1_in_idx), (and_1_idx, and_a_idx))
            .unwrap();
        netlist
            .connect((in_2_idx, in_2_in_idx), (and_1_idx, and_b_idx))
            .unwrap();
        netlist
            .connect((in_3_idx, in_3_in_idx), (and_2_idx, and_a_idx))
            .unwrap();
        netlist
            .connect((in_4_idx, in_4_in_idx), (and_2_idx, and_b_idx))
            .unwrap();

        netlist
            .connect((and_1_idx, and_out_idx), (or_idx, or_a_idx))
            .unwrap();
        netlist
            .connect((and_2_idx, and_out_idx), (or_idx, or_b_idx))
            .unwrap();

        netlist
            .connect((or_idx, or_out_idx), (out_idx, out_out_idx))
            .unwrap();
    }
}
