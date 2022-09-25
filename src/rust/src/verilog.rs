use std::collections::HashSet;

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
    ArithmeticShift,
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

pub enum PortDirection {
    Input,
    Output,
    Inout,
}

pub struct Port {
    name: String,
    direction: PortDirection,
    kind: VerilogKind,
}

pub struct Module {
    name: String,
    ports: HashSet<Port>,
}
