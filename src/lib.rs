//! This module contians the functions that we'll extern out to C, to be interacted with from the GUI code
pub mod verilog;

use crate::{
    ffi::{SizedVerilogKind, UnsizedVerilogKind},
    verilog::{Module, ModuleIndex, Netlist, Port, VerilogKind},
};
use ffi::CModIndex;
use generational_arena::Index;
use lazy_static::lazy_static;
use std::sync::Mutex;
use verilog::PortIndex;

lazy_static! {
    /// Global netlist that we'll refer to
    pub static ref NETLIST: Mutex<Netlist> = Mutex::new(Netlist::new());
}

#[cxx::bridge(namespace = "org::cfrs")]
mod ffi {
    // Shared types
    #[derive(Debug)]
    pub enum UnsizedVerilogKind {
        Byte,
        ShortInteger,
        Integer,
        LongInteger,
        Time,
        ShortReal,
        Real,
    }

    #[derive(Debug)]
    pub enum SizedVerilogKind {
        Bit,
        Logic,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct CModIndex {
        index: usize,
        generation: u64,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type ModuleIndex;
        fn add_new_module(name: String) -> CModIndex;
        fn add_sized_input_port(name: String, kind: SizedVerilogKind, idx: CModIndex, size: usize);
        fn add_sized_output_port(name: String, kind: SizedVerilogKind, idx: CModIndex, size: usize);
        fn add_unsized_input_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex);
        fn add_unsized_output_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex);
        fn dump_netlist();
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

// C++ interface

pub fn add_input_port(name: String, kind: VerilogKind, idx: ModuleIndex) -> Option<PortIndex> {
    add_port(Port::input(name, kind), idx)
}

pub fn add_output_port(name: String, kind: VerilogKind, idx: ModuleIndex) -> Option<PortIndex> {
    add_port(Port::output(name, kind), idx)
}

pub fn add_unsized_input_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex) {
    add_input_port(name, kind.to_verilog_kind(), idx.to_module_index());
}

pub fn add_sized_input_port(name: String, kind: SizedVerilogKind, idx: CModIndex, size: usize) {
    add_input_port(name, kind.to_verilog_kind(size), idx.to_module_index());
}

pub fn add_unsized_output_port(name: String, kind: UnsizedVerilogKind, idx: CModIndex) {
    add_output_port(name, kind.to_verilog_kind(), idx.to_module_index());
}

pub fn add_sized_output_port(name: String, kind: SizedVerilogKind, idx: CModIndex, size: usize) {
    add_output_port(name, kind.to_verilog_kind(size), idx.to_module_index());
}

impl UnsizedVerilogKind {
    pub fn to_verilog_kind(&self) -> VerilogKind {
        match *self {
            UnsizedVerilogKind::Byte => VerilogKind::Byte,
            UnsizedVerilogKind::ShortInteger => VerilogKind::ShortInteger,
            UnsizedVerilogKind::Integer => VerilogKind::Integer,
            UnsizedVerilogKind::LongInteger => VerilogKind::LongInteger,
            UnsizedVerilogKind::Time => VerilogKind::Time,
            UnsizedVerilogKind::ShortReal => VerilogKind::ShortReal,
            UnsizedVerilogKind::Real => VerilogKind::Real,
            _ => unreachable!(),
        }
    }
    pub fn from_verilog_kind(vk: VerilogKind) -> Self {
        match vk {
            VerilogKind::Byte => UnsizedVerilogKind::Byte,
            VerilogKind::ShortInteger => UnsizedVerilogKind::ShortInteger,
            VerilogKind::Integer => UnsizedVerilogKind::Integer,
            VerilogKind::LongInteger => UnsizedVerilogKind::LongInteger,
            VerilogKind::Time => UnsizedVerilogKind::Time,
            VerilogKind::ShortReal => UnsizedVerilogKind::ShortReal,
            VerilogKind::Real => UnsizedVerilogKind::Real,
            _ => unreachable!(),
        }
    }
}

impl SizedVerilogKind {
    pub fn to_verilog_kind(&self, size: usize) -> VerilogKind {
        match *self {
            SizedVerilogKind::Bit => VerilogKind::Bit(size),
            SizedVerilogKind::Logic => VerilogKind::Logic(size),
            _ => unreachable!(),
        }
    }
    pub fn from_verilog_kind(vk: VerilogKind) -> Self {
        match vk {
            VerilogKind::Bit(_) => Self::Bit,
            VerilogKind::Logic(_) => Self::Logic,
            _ => unreachable!(),
        }
    }
}

/// Print a debug output of the netlist to stdout
pub fn dump_netlist() {
    let netlist = NETLIST.lock().expect("Lock won't panic");
    println!("{:#?}", &*netlist);
}
