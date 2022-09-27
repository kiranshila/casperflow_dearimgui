use std::fmt::Display;

use generational_arena::{Arena, Index};

use crate::ffi::ConnectionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleIndex(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortIndex(pub Index);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
/// These are the verilog types that must be driven. The value channges
/// when the driver changes value. These identifiers represent wires.
pub enum VerilogNetKind {
    /// Simple interconnecting wire
    Wire,
    /// Wired outputs OR together
    WOr,
    /// Wired outputs AND together
    WAnd,
    /// Pulls down when TriStated
    Tri0,
    /// Pulls up when TriStated
    Tri1,
    /// Constant Logic 0
    Supply0,
    /// Constant Logic 1,
    Supply1,
    /// Stores the lasts value when TriStated
    TriReg,
}

impl Default for VerilogNetKind {
    fn default() -> Self {
        VerilogNetKind::Wire
    }
}

// TODO: Strength

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
/// An identifier of "variable data type" means that it changes value upon
/// assignment and holds its value until another assignment.
pub enum VerilogVariableKind {
    /// Signed 32-bit variable
    Integer,
    /// Double-precision floating point
    Real,
    /// Any bit size or signedness
    Reg { signed: bool, size: usize },
    /// Unsigned 64-bit variable
    Time,
}

impl Default for VerilogVariableKind {
    fn default() -> Self {
        VerilogVariableKind::Reg {
            signed: false,
            size: 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum VerilogKind {
    Net {
        kind: VerilogNetKind,
        size: usize,
        signed: bool,
    },
    Variable(VerilogVariableKind),
}

impl Default for VerilogKind {
    fn default() -> Self {
        VerilogKind::Net {
            kind: Default::default(),
            size: 1,
            signed: false,
        }
    }
}

impl Display for VerilogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerilogKind::Net { kind, size, signed } => {
                let arr = if *size == 1usize {
                    if *signed {
                        format!(" signed ")
                    } else {
                        format!("")
                    }
                } else {
                    if *signed {
                        format!(" signed [{}:{}]", size - 1, 0)
                    } else {
                        format!("[{}:{}]", size - 1, 0)
                    }
                };
                match kind {
                    VerilogNetKind::Wire => write!(f, "wire{}", arr),
                    VerilogNetKind::WOr => write!(f, "wor{}", arr),
                    VerilogNetKind::WAnd => write!(f, "wand{}", arr),
                    VerilogNetKind::Tri0 => write!(f, "tri0{}", arr),
                    VerilogNetKind::Tri1 => write!(f, "tri1{}", arr),
                    VerilogNetKind::Supply0 => write!(f, "supply0{}", arr),
                    VerilogNetKind::Supply1 => write!(f, "supply1{}", arr),
                    VerilogNetKind::TriReg => write!(f, "trireg{}", arr),
                }
            }
            VerilogKind::Variable(kind) => match kind {
                VerilogVariableKind::Integer => write!(f, "integer"),
                VerilogVariableKind::Real => write!(f, "real"),
                VerilogVariableKind::Reg { signed, size } => {
                    if *signed {
                        write!(
                            f,
                            "reg signed {}",
                            if *size == 1usize {
                                format!("")
                            } else {
                                format!("[{}:{}]", size - 1, 0)
                            }
                        )
                    } else {
                        write!(
                            f,
                            "reg{}",
                            if *size == 1usize {
                                format!("")
                            } else {
                                format!("[{}:{}]", size - 1, 0)
                            }
                        )
                    }
                }
                VerilogVariableKind::Time => todo!(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerilogPrimitive {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    // Bitwise
    Not,
    Or,
    And,
    Xor,
    Nand,
    // Relational
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
    // Logical
    Negation,
    LogicalOr,
    LogicalAnd,
    // Shift,
    LogicalRightShift,
    ArithmeticRightShift,
    LeftShift,
    // Concatenation
    Concatenation,
    Replication,
    // Conditional
    Conditional,
    // Sign Formatting
    ToUnsigned,
    ToSigned,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Port {
    Input {
        name: String,
        kind: VerilogKind,
        connection: Option<PortIndex>,
    },
    Output {
        name: String,
        kind: VerilogKind,
        connections: Vec<PortIndex>,
    },
}

impl Clone for Port {
    fn clone(&self) -> Self {
        match self {
            Self::Input { name, kind, .. } => Self::Input {
                name: name.clone(),
                kind: kind.clone(),
                connection: None,
            },
            Self::Output { name, kind, .. } => Self::Output {
                name: name.clone(),
                kind: kind.clone(),
                connections: vec![],
            },
        }
    }
}

impl Port {
    pub fn input(name: String, kind: VerilogKind) -> Port {
        Port::Input {
            name,
            kind,
            connection: None,
        }
    }
    pub fn output(name: String, kind: VerilogKind) -> Port {
        Port::Output {
            name,
            kind,
            connections: vec![],
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Port::Input { name, .. } => name,
            Port::Output { name, .. } => name,
        }
    }
    pub fn kind(&self) -> VerilogKind {
        match self {
            Port::Input { kind, .. } => *kind,
            Port::Output { kind, .. } => *kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub inputs: Vec<PortIndex>,
    pub outputs: Vec<PortIndex>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            inputs: vec![],
            outputs: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Netlist {
    modules: Arena<Module>,
    ports: Arena<Port>,
    wires: Vec<(PortIndex, PortIndex)>,
}

impl Netlist {
    pub fn new() -> Self {
        Self {
            modules: Arena::new(),
            ports: Arena::new(),
            wires: vec![],
        }
    }

    pub fn add_module(&mut self, module: Module) -> ModuleIndex {
        ModuleIndex(self.modules.insert(module))
    }

    pub fn remove_module(&mut self, idx: ModuleIndex) -> Option<Module> {
        // Remove the module
        let m = self.modules.remove(idx.0)?;
        // Remove all the associated ports
        for pi in m.inputs.iter() {
            self.ports.remove(pi.0);
        }
        for pi in m.outputs.iter() {
            self.ports.remove(pi.0);
        }
        Some(m)
    }

    pub fn modules(&self) -> generational_arena::Iter<Module> {
        self.modules.iter()
    }

    pub fn wires(&self) -> std::slice::Iter<(PortIndex, PortIndex)> {
        self.wires.iter()
    }

    pub fn add_port(&mut self, port: Port, idx: ModuleIndex) -> Option<PortIndex> {
        let is_input = matches!(port, Port::Input { .. });
        let pi = PortIndex(self.ports.insert(port));
        let m = self.modules.get_mut(idx.0)?;
        if is_input {
            m.inputs.push(pi)
        } else {
            m.outputs.push(pi)
        }
        Some(pi)
    }

    pub fn remove_input_port(&mut self, mod_idx: ModuleIndex, port_idx: usize) -> Option<Port> {
        // Grab the module by index
        let m = self.modules.get(mod_idx.0)?;
        // Grab the specific port index
        let pi = m.inputs.get(port_idx)?;
        // Remove the port
        self.ports.remove(pi.0)
    }

    pub fn remove_output_port(&mut self, mod_idx: ModuleIndex, port_idx: usize) -> Option<Port> {
        // Grab the module by index
        let m = self.modules.get(mod_idx.0)?;
        // Grab the specific port index
        let pi = m.outputs.get(port_idx)?;
        // Remove the port
        self.ports.remove(pi.0)
    }

    pub fn get_module(&self, idx: ModuleIndex) -> Option<&Module> {
        self.modules.get(idx.0)
    }

    pub fn get_mut_module(&mut self, idx: ModuleIndex) -> Option<&mut Module> {
        self.modules.get_mut(idx.0)
    }

    pub fn get_port(&self, idx: PortIndex) -> Option<&Port> {
        self.ports.get(idx.0)
    }

    pub fn connect(
        &mut self,
        output_mod: ModuleIndex,
        output_port: usize,
        input_mod: ModuleIndex,
        input_port: usize,
    ) -> Result<usize, ConnectionResult> {
        // Check that input and output are different
        if input_mod == output_mod {
            return Err(ConnectionResult::BadIndex);
        }
        // Get the modules
        let (in_mod, out_mod) = self.modules.get2_mut(input_mod.0, output_mod.0);

        // Get the port indices
        let in_port_idx = in_mod.and_then(|m| m.inputs.get(input_port));
        let out_port_idx = out_mod.and_then(|m| m.outputs.get(output_port));

        let in_port_idx = match in_port_idx {
            Some(pi) => pi,
            None => return Err(ConnectionResult::BadIndex),
        };

        let out_port_idx = match out_port_idx {
            Some(pi) => pi,
            None => return Err(ConnectionResult::BadIndex),
        };

        // Grab the ports
        let (in_port, out_port) = self.ports.get2_mut(in_port_idx.0, out_port_idx.0);

        // Check directions and existance
        let in_port = match in_port {
            Some(port) if matches!(port, Port::Input { .. }) => port,
            Some(_) => return Err(ConnectionResult::DirectionMismatch),
            None => return Err(ConnectionResult::BadIndex),
        };
        let out_port = match out_port {
            Some(port) if matches!(port, Port::Output { .. }) => port,
            Some(_) => return Err(ConnectionResult::DirectionMismatch),
            None => return Err(ConnectionResult::BadIndex),
        };

        // Check to make sure the types match
        if in_port.kind() != out_port.kind() {
            return Err(ConnectionResult::TypeMismatch);
        }

        // Check to verify inputs have only one connection
        if matches!(
            in_port,
            Port::Input {
                connection: Some(_),
                ..
            }
        ) {
            return Err(ConnectionResult::InputDriven);
        }

        // Drive the input
        if let Port::Input {
            ref mut connection, ..
        } = in_port
        {
            *connection = Some(*out_port_idx);
        }

        // Add to the list of driving inputs to the output
        if let Port::Output {
            ref mut connections,
            ..
        } = out_port
        {
            connections.push(*in_port_idx);
        }

        // Add the wire
        self.wires.push((*in_port_idx, *out_port_idx));

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
        // Create the netlist
        let mut netlist = Netlist::new();

        // Add some Modules
        let and_1 = netlist.add_module(Module::new("and".to_owned()));
        netlist.add_port(Port::input("A".to_owned(), Default::default()), and_1);
        netlist.add_port(Port::input("B".to_owned(), Default::default()), and_1);
        netlist.add_port(Port::output("Out".to_owned(), Default::default()), and_1);

        let and_2 = netlist.add_module(Module::new("and".to_owned()));
        netlist.add_port(Port::input("A".to_owned(), Default::default()), and_2);
        netlist.add_port(Port::input("B".to_owned(), Default::default()), and_2);
        netlist.add_port(Port::output("Out".to_owned(), Default::default()), and_2);

        let or = netlist.add_module(Module::new("or".to_owned()));
        netlist.add_port(Port::input("A".to_owned(), Default::default()), or);
        netlist.add_port(Port::input("B".to_owned(), Default::default()), or);
        netlist.add_port(Port::output("Out".to_owned(), Default::default()), or);

        // Some "external" ports
        let in_1 = netlist.add_module(Module::new("In".to_owned()));
        netlist.add_port(Port::output("In".to_owned(), Default::default()), in_1);
        let in_2 = netlist.add_module(Module::new("In".to_owned()));
        netlist.add_port(Port::output("In".to_owned(), Default::default()), in_2);
        let in_3 = netlist.add_module(Module::new("In".to_owned()));
        netlist.add_port(Port::output("In".to_owned(), Default::default()), in_3);
        let in_4 = netlist.add_module(Module::new("In".to_owned()));
        netlist.add_port(Port::output("In".to_owned(), Default::default()), in_4);

        let out = netlist.add_module(Module::new("Out".to_owned()));
        netlist.add_port(Port::input("Out".to_owned(), Default::default()), out);

        // Make connections, none of these should fail
        netlist.connect(in_1, 0, and_1, 0).unwrap();
        netlist.connect(in_2, 0, and_1, 1).unwrap();

        netlist.connect(in_3, 0, and_2, 0).unwrap();
        netlist.connect(in_4, 0, and_2, 1).unwrap();

        netlist.connect(and_1, 0, or, 0).unwrap();
        netlist.connect(and_2, 0, or, 1).unwrap();

        netlist.connect(or, 0, out, 0).unwrap();
    }
}
