//! This module contians the functions that we'll extern out to C, to be interacted with from the GUI code
pub mod verilog;

use crate::{
    ffi::{SizedVerilogKind, UnsizedVerilogKind},
    verilog::{Module, ModuleIndex, Netlist, Port, VerilogKind, WireIndex},
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Global netlist that we'll refer to
    pub static ref NETLIST: Mutex<Netlist> = Mutex::new(Netlist::new());
    /// Global counter to generate unique indices for modules
    pub static ref NEXT_MOD_IDX: Mutex<i32> = Mutex::new(0);
    /// Global counter to generate unique indices for pins
    pub static ref NEXT_PIN_IDX: Mutex<i32> = Mutex::new(0);
    /// Vecs from imnodes idx to a generational index
    pub static ref MODS: Mutex<Vec<ModuleIndex>> = Mutex::new(vec![]);
    pub static ref PINS: Mutex<Vec<WireIndex>> = Mutex::new(vec![]);
}

#[cxx::bridge(namespace = "org::crfs")]
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

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        fn add_new_module(name: String) -> i32;
        fn add_unsized_input_port(name: String, mod_idx: i32, kind: UnsizedVerilogKind) -> i32;
        fn add_sized_input_port(
            name: String,
            mod_idx: i32,
            kind: SizedVerilogKind,
            size: usize,
        ) -> i32;
        fn dump_netlist();
    }
}

/// Add a new module with `name` to the global netlist
pub fn add_new_module(name: String) -> i32 {
    // Create the module instance
    let new_mod = Module::new(name);
    // Add the module to the netlist
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    // Get our new real index
    let idx = (*netlist).add_module(new_mod);
    // Grab the next valid id
    let mut next_id = NEXT_MOD_IDX.lock().expect("Lock won't panic");
    let raw_idx = *next_id;
    // Increment the counter
    *next_id += 1;
    // Add the real index to the mods
    let mut mods = MODS.lock().expect("Lock won't panic");
    (*mods).push(idx);
    // Return the id to imnodes
    raw_idx
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

fn add_port(mod_idx: i32, port: Port) -> i32 {
    // Grab the module indicated by the mod_idx
    let mods = MODS.lock().expect("Lock won't panic");
    let mi = match mods.get(mod_idx as usize) {
        Some(m) => m,
        None => return -1,
    };
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let m = match (*netlist).get_mut_module(*mi) {
        Some(m_ref) => m_ref,
        None => return -1,
    };
    // Add this port to the module
    let port_index = m.add_port(port);
    // Create the wire index of this
    let idx = (*mi, port_index);
    // Grab the next valid port id
    let mut next_id = NEXT_PIN_IDX.lock().expect("Lock won't panic");
    let raw_idx = *next_id;
    // Increment the counter
    *next_id += 1;
    // Add the real index to the pins
    let mut mods = PINS.lock().expect("Lock won't panic");
    (*mods).push(idx);
    // Return the id to imnodes
    raw_idx
}

fn add_input_port(name: String, mod_idx: i32, kind: VerilogKind) -> i32 {
    // Create the new port
    let port = Port::input(name, kind);
    add_port(mod_idx, port)
}

fn add_output_port(name: String, mod_idx: i32, kind: VerilogKind) -> i32 {
    // Create the new port
    let port = Port::output(name, kind);
    add_port(mod_idx, port)
}

/// Add an unsized input port with `name` and `kind` to the module indicated by `mod_idx`.
pub fn add_unsized_input_port(name: String, mod_idx: i32, kind: UnsizedVerilogKind) -> i32 {
    add_input_port(name, mod_idx, kind.to_verilog_kind())
}

/// Add a sized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
pub fn add_sized_input_port(
    name: String,
    mod_idx: i32,
    kind: SizedVerilogKind,
    size: usize,
) -> i32 {
    add_input_port(name, mod_idx, kind.to_verilog_kind(size))
}

/// Add an unnsized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
pub fn add_unsized_output_port(name: String, mod_idx: i32, kind: UnsizedVerilogKind) -> i32 {
    add_output_port(name, mod_idx, kind.to_verilog_kind())
}

/// Add a sized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
pub fn add_sized_output_port(
    name: String,
    mod_idx: i32,
    kind: SizedVerilogKind,
    size: usize,
) -> i32 {
    add_output_port(name, mod_idx, kind.to_verilog_kind(size))
}

/// Print a debug output of the netlist to stdout
pub fn dump_netlist() {
    let netlist = NETLIST.lock().expect("Lock won't panic");
    println!("{:#?}", &*netlist);
}
