//! This module contians the functions that we'll extern out to C, to be interacted with from the GUI code
pub mod verilog;

use crate::{
    ffi::{SizedVerilogKind, UnsizedVerilogKind},
    verilog::{Module, ModuleIndex, Netlist, Port, VerilogKind},
};
use bimap::BiMap;
use ffi::{CGraph, CModIndex, CModule, CPort, CWire, ConnectionResult};
use generational_arena::Index;
use lazy_static::lazy_static;
use std::sync::Mutex;
use verilog::{PortIndex, VerilogNetKind, VerilogVariableKind};

lazy_static! {
    /// Global netlist that we'll refer to
    pub static ref NETLIST: Mutex<Netlist> = Mutex::new(Netlist::new());
    /// Wires ids are just their index in the netlist, but pins are computed during drawing, we'll use a hashmap to keep track
    pub static ref PORT_MAP: Mutex<BiMap<PortIndex,i32>> = Mutex::new(BiMap::new());
    // Same goes for module id
    pub static ref MOD_MAP: Mutex<BiMap<ModuleIndex,i32>> = Mutex::new(BiMap::new());
}

#[cxx::bridge(namespace = "org::cfrs")]
mod ffi {
    // Shared types
    #[derive(Debug)]
    pub enum UnsizedVerilogKind {
        Integer,
        Real,
        Time,
    }

    #[derive(Debug)]
    pub enum SizedVerilogKind {
        Wire,
        WOr,
        WAnd,
        Tri0,
        Tri1,
        Supply0,
        Supply1,
        TriReg,
        Reg,
    }

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct CModIndex {
        index: usize,
        generation: u64,
    }

    #[derive(Debug)]
    pub struct CPort {
        id: i32,
        name: String,
    }

    #[derive(Debug)]
    pub struct CModule {
        id: i32,
        name: String,
        inputs: Vec<CPort>,
        outputs: Vec<CPort>,
    }

    #[derive(Debug)]
    pub struct CWire {
        id: i32,
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    pub struct CGraph {
        modules: Vec<CModule>,
        wires: Vec<CWire>,
    }

    #[derive(Debug)]
    pub enum ConnectionResult {
        BadIndex,
        DirectionMismatch,
        TypeMismatch,
        InputDriven,
        ConnectionOk,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        fn add_new_module(name: String) -> CModIndex;
        fn add_sized_input_port(
            name: String,
            kind: SizedVerilogKind,
            idx: CModIndex,
            size: usize,
            signed: bool,
        );
        fn add_sized_output_port(
            name: String,
            kind: SizedVerilogKind,
            idx: CModIndex,
            size: usize,
            signed: bool,
        );
        fn add_unsized_input_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex);
        fn add_unsized_output_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex);
        fn dump_netlist();
        fn connect(
            output_mod: CModIndex,
            output_port: usize,
            input_mod: CModIndex,
            input_port: usize,
        ) -> ConnectionResult;
        fn get_graph() -> CGraph;
    }
}

impl CModIndex {
    pub fn to_module_index(&self) -> ModuleIndex {
        ModuleIndex(Index::from_raw_parts(self.index, self.generation))
    }
    pub fn from_module_index(idx: ModuleIndex) -> Self {
        let (index, generation) = idx.0.into_raw_parts();
        Self { index, generation }
    }
}

/// Add a new module with `name` to the global netlist
pub fn add_new_module(name: String) -> CModIndex {
    // Create the module instance
    let new_mod = Module::new(name);
    // Add the module to the netlist
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    CModIndex::from_module_index((*netlist).add_module(new_mod))
}

/// Wraps netlist.add_port with the global netlist
pub fn add_port(port: Port, idx: ModuleIndex) -> Option<PortIndex> {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    netlist.add_port(port, idx)
}

pub fn add_input_port(name: String, kind: VerilogKind, idx: ModuleIndex) -> Option<PortIndex> {
    add_port(Port::input(name, kind), idx)
}

pub fn add_output_port(name: String, kind: VerilogKind, idx: ModuleIndex) -> Option<PortIndex> {
    add_port(Port::output(name, kind), idx)
}

pub fn add_unsized_input_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex) {
    add_input_port(name, kind.to_verilog_kind(), idx.to_module_index());
}

pub fn add_sized_input_port(
    name: String,
    kind: SizedVerilogKind,
    idx: CModIndex,
    size: usize,
    signed: bool,
) {
    add_input_port(
        name,
        kind.to_verilog_kind(size, signed),
        idx.to_module_index(),
    );
}

pub fn add_unsized_output_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex) {
    add_output_port(name, kind.to_verilog_kind(), idx.to_module_index());
}

pub fn add_sized_output_port(
    name: String,
    kind: SizedVerilogKind,
    idx: CModIndex,
    size: usize,
    signed: bool,
) {
    add_output_port(
        name,
        kind.to_verilog_kind(size, signed),
        idx.to_module_index(),
    );
}

impl UnsizedVerilogKind {
    pub fn to_verilog_kind(&self) -> VerilogKind {
        match *self {
            UnsizedVerilogKind::Integer => VerilogKind::Variable(VerilogVariableKind::Integer),
            UnsizedVerilogKind::Real => VerilogKind::Variable(VerilogVariableKind::Real),
            UnsizedVerilogKind::Time => VerilogKind::Variable(VerilogVariableKind::Time),
            _ => unreachable!(),
        }
    }
    pub fn from_verilog_kind(vk: VerilogKind) -> Self {
        match vk {
            VerilogKind::Variable(v) => match v {
                verilog::VerilogVariableKind::Integer => UnsizedVerilogKind::Integer,
                verilog::VerilogVariableKind::Real => UnsizedVerilogKind::Real,
                verilog::VerilogVariableKind::Time => UnsizedVerilogKind::Time,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl SizedVerilogKind {
    pub fn to_verilog_kind(&self, size: usize, signed: bool) -> VerilogKind {
        match *self {
            SizedVerilogKind::Wire => VerilogKind::Net {
                kind: VerilogNetKind::Wire,
                size,
                signed,
            },
            SizedVerilogKind::WOr => VerilogKind::Net {
                kind: VerilogNetKind::WOr,
                size,
                signed,
            },
            SizedVerilogKind::WAnd => VerilogKind::Net {
                kind: VerilogNetKind::WAnd,
                size,
                signed,
            },
            SizedVerilogKind::Tri0 => VerilogKind::Net {
                kind: VerilogNetKind::Tri0,
                size,
                signed,
            },
            SizedVerilogKind::Tri1 => VerilogKind::Net {
                kind: VerilogNetKind::Tri1,
                size,
                signed,
            },
            SizedVerilogKind::Supply0 => VerilogKind::Net {
                kind: VerilogNetKind::Supply0,
                size,
                signed,
            },
            SizedVerilogKind::Supply1 => VerilogKind::Net {
                kind: VerilogNetKind::Supply1,
                size,
                signed,
            },
            SizedVerilogKind::TriReg => VerilogKind::Net {
                kind: VerilogNetKind::TriReg,
                size,
                signed,
            },
            SizedVerilogKind::Reg => {
                VerilogKind::Variable(VerilogVariableKind::Reg { signed, size })
            }
            _ => unreachable!(),
        }
    }
    pub fn from_verilog_kind(vk: VerilogKind) -> Self {
        match vk {
            VerilogKind::Net { kind, .. } => match kind {
                verilog::VerilogNetKind::Wire => SizedVerilogKind::Wire,
                verilog::VerilogNetKind::WOr => SizedVerilogKind::WOr,
                verilog::VerilogNetKind::WAnd => SizedVerilogKind::WAnd,
                verilog::VerilogNetKind::Tri0 => SizedVerilogKind::Tri0,
                verilog::VerilogNetKind::Tri1 => SizedVerilogKind::Tri1,
                verilog::VerilogNetKind::Supply0 => SizedVerilogKind::Supply0,
                verilog::VerilogNetKind::Supply1 => SizedVerilogKind::Supply1,
                verilog::VerilogNetKind::TriReg => SizedVerilogKind::TriReg,
            },
            VerilogKind::Variable(kind) => match kind {
                VerilogVariableKind::Reg { .. } => SizedVerilogKind::Reg,
                _ => unreachable!(),
            },
        }
    }
}

/// Print a debug output of the netlist to stdout
pub fn dump_netlist() {
    let netlist = NETLIST.lock().expect("Lock won't panic");
    println!("{:#?}", &*netlist);
}

pub fn get_graph() -> CGraph {
    // Grab the globls
    let netlist = NETLIST.lock().expect("Lock won't panic");
    let mut mod_map = MOD_MAP.lock().expect("Lock won't panic");
    let mut port_map = PORT_MAP.lock().expect("Lock won't panic");

    // Clear all our old drawing state
    (*mod_map).clear();
    (*port_map).clear();

    // Counter for the ports
    let mut port_id = 0i32;

    // Grab the modules
    let modules = (*netlist)
        .modules()
        .enumerate()
        .map(|(id, (_, m))| {
            let id = id as i32;
            CModule {
                id,
                name: m.name.to_owned(),
                inputs: m
                    .inputs
                    .iter()
                    .map(|x| {
                        let port = netlist.get_port(*x).expect("These will always be valid");
                        let id = port_id;
                        let name = port.name().to_owned();
                        // Increment the global id counter
                        port_id += 1;
                        // Create the lookup
                        port_map.insert(*x, id);
                        CPort { id, name }
                    })
                    .collect(),
                outputs: m
                    .outputs
                    .iter()
                    .map(|x| {
                        let port = netlist.get_port(*x).expect("These will always be valid");
                        let id = port_id;
                        let name = port.name().to_owned();
                        // Increment the global id counter
                        port_id += 1;
                        // Create the lookup
                        port_map.insert(*x, id);
                        CPort { id, name }
                    })
                    .collect(),
            }
        })
        .collect();
    // Grab the ports
    let wires = (*netlist)
        .wires()
        .enumerate()
        .map(|(id, (x, y))| CWire {
            id: id as i32,
            x: *(*port_map).get_by_left(x).unwrap(),
            y: *(*port_map).get_by_left(y).unwrap(),
        })
        .collect();
    CGraph { modules, wires }
}

pub fn connect(
    output_mod: CModIndex,
    output_port: usize,
    input_mod: CModIndex,
    input_port: usize,
) -> ConnectionResult {
    // Grab the netlist
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    match netlist.connect(
        output_mod.to_module_index(),
        output_port,
        input_mod.to_module_index(),
        input_port,
    ) {
        Ok(_) => ConnectionResult::ConnectionOk,
        Err(e) => e,
    }
}
