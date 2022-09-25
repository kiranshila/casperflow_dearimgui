use generational_arena::{Arena, Index};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum VerilogKind {
    Bit(usize),
    Byte,
    ShortInteger,
    Integer,
    LongInteger,
    Logic(usize),
    Time,
    ShortReal,
    Real,
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
        connection: Option<WireIndex>,
    },
    Output {
        name: String,
        kind: VerilogKind,
        connections: Vec<WireIndex>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleIndex(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortIndex(pub Index);

pub type WireIndex = (ModuleIndex, PortIndex);

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub ports: Arena<Port>,
    pub port_map: HashMap<String, PortIndex>,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ports: Arena::new(),
            port_map: HashMap::new(),
        }
    }
    pub fn add_port(&mut self, port: Port) -> PortIndex {
        // Find if a port with this name already exists and remove it
        let key = port.name().to_owned();
        if let Some(idx) = self.port_map.get(&key) {
            self.ports.remove(idx.0);
        }
        // Insert into the ports
        let new_idx = PortIndex(self.ports.insert(port));
        self.port_map.insert(key, new_idx);
        new_idx
    }
}
