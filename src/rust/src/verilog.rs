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

// Use the Szudzik (2006) pairing function to encode two numbers
fn to_szudzik<T1, T2>(x: T1, y: T2) -> i32
where
    i32: From<T1> + From<T2>,
{
    let x = i32::from(x);
    let y = i32::from(y);

    if x > y {
        x.pow(2) + x + y
    } else {
        y.pow(2) + x
    }
}

fn from_szudzik(z: i32) -> (i32, i32) {
    let frootz = (z as f32).sqrt().floor() as i32;
    let frootz_sq = frootz.pow(2) as i32;
    if z - frootz_sq < frootz {
        (z - frootz_sq, frootz)
    } else {
        (frootz, z - frootz_sq)
    }
}

fn index_to_szudzik(idx: Index) -> i32 {
    let (x, y) = idx.into_raw_parts();
    let x = x as i32;
    let y = y as i32;
    to_szudzik(x, y)
}

fn index_from_szudzik(z: i32) -> Index {
    let (x, y) = from_szudzik(z);
    Index::from_raw_parts(x as usize, y as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleIndex(pub Index);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortIndex(pub Index);

impl ModuleIndex {
    fn to_szudzik(&self) -> i32 {
        index_to_szudzik(self.0)
    }
    fn from_szudzik(z: i32) -> Self {
        ModuleIndex(index_from_szudzik(z))
    }
}

impl PortIndex {
    fn to_szudzik(&self) -> i32 {
        index_to_szudzik(self.0)
    }
    fn from_szudzik(z: i32) -> Self {
        PortIndex(index_from_szudzik(z))
    }
}

pub type WireIndex = (ModuleIndex, PortIndex);

fn wire_index_to_szudzik(idx: WireIndex) -> i32 {
    let (mi, pi) = idx;
    let mic = mi.to_szudzik();
    let pic = pi.to_szudzik();
    to_szudzik(mic, pic)
}

fn wire_index_from_szudzik(z: i32) -> WireIndex {
    let (x, y) = from_szudzik(z);
    let mi = ModuleIndex::from_szudzik(x);
    let pi = PortIndex::from_szudzik(y);
    (mi, pi)
}

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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_szudzik() {
        let x = 8174;
        let y = 1782;
        let z = to_szudzik(x, y);
        assert_eq!((x, y), from_szudzik(z));
    }

    #[test]
    fn test_indexing_szudzik() {
        let idx_a = ModuleIndex(Index::from_raw_parts(100, 200));
        let idx_b = PortIndex(Index::from_raw_parts(300, 400));
        let idx = (idx_a, idx_b);

        let z = wire_index_to_szudzik(idx);
        dbg!(z);
        assert_eq!(idx, wire_index_from_szudzik(z));
    }
}
