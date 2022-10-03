//! This module contians the functions that we'll extern out to C, to be interacted with from the GUI code
pub mod library;
pub mod netlist;

use crate::netlist::{ModuleIndex, Netlist, PinIndex, WireIndex};
use anyhow::anyhow;
use bimap::BiMap;
use ffi::{CGraph, CModule, CPort, CWire, InterconnectDirection, PinKind};
use lazy_static::lazy_static;
use std::{fs::File, io::Read, sync::Mutex};

lazy_static! {
    /// Global netlist that we'll refer to
    pub static ref NETLIST: Mutex<Netlist> = Mutex::new(Netlist::new());
    pub static ref PIN_MAP: Mutex<BiMap<PinIndex,i32>> = Mutex::new(BiMap::new());
    pub static ref MOD_MAP: Mutex<BiMap<ModuleIndex,i32>> = Mutex::new(BiMap::new());
    pub static ref WIRE_MAP: Mutex<BiMap<WireIndex,i32>> = Mutex::new(BiMap::new());
}

#[cxx::bridge(namespace = "org::cfrs")]
mod ffi {

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum InterconnectDirection {
        Input,
        Output,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PinKind {
        Wire,
        Integer,
        Real,
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
        position: [f32; 2],
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

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        fn add_module(name: String);
        fn remove_module(mod_id: i32) -> i32;
        fn add_pin(
            mod_id: i32,
            name: String,
            kind: PinKind,
            direction: InterconnectDirection,
        ) -> i32;
        fn remove_pin(pin_id: i32) -> i32;
        fn add_wire(in_a_id: i32, in_b_id: i32) -> Result<()>;
        fn remove_wire(wire_id: i32) -> i32;
        fn set_module_position(mod_id: i32, x: f32, y: f32) -> i32;

        fn get_graph() -> CGraph;
        fn dump_netlist();

        fn add_module_from_json_path(path: String, x: f32, y: f32) -> Result<()>;
        fn get_json_module(mod_id: i32) -> String;
    }
}

pub fn add_module(name: String) {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    netlist.add_module(name);
}

pub fn remove_module(mod_id: i32) -> i32 {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let mod_map = MOD_MAP.lock().expect("Lock won't panic");
    // Get mod index from id
    let m = if let Some(m) = mod_map.get_by_right(&mod_id) {
        m
    } else {
        return -1;
    };
    if netlist.remove_module(*m).is_none() {
        -1
    } else {
        0
    }
}

pub fn add_pin(mod_id: i32, name: String, kind: PinKind, direction: InterconnectDirection) -> i32 {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let mod_map = MOD_MAP.lock().expect("Lock won't panic");
    // Get mod index from id
    let m = if let Some(m) = mod_map.get_by_right(&mod_id) {
        m
    } else {
        return -1;
    };
    if netlist.add_pin(*m, name, kind, direction).is_none() {
        -1
    } else {
        0
    }
}

pub fn remove_pin(pin_id: i32) -> i32 {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let pin_map = PIN_MAP.lock().expect("Lock won't panic");
    // Get pin index from id
    let m = if let Some(m) = pin_map.get_by_right(&pin_id) {
        m
    } else {
        return -1;
    };
    if netlist.remove_pin(*m).is_none() {
        -1
    } else {
        0
    }
}

fn add_wire(in_a_id: i32, in_b_id: i32) -> anyhow::Result<()> {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let pin_map = PIN_MAP.lock().expect("Lock won't panic");
    // Get pin indices from ids
    let a_idx = pin_map
        .get_by_right(&in_a_id)
        .ok_or(anyhow!("Pin a not found"))?;
    let b_idx = pin_map
        .get_by_right(&in_b_id)
        .ok_or(anyhow!("Pin b not found"))?;
    // Try to connect
    netlist.add_wire(*a_idx, *b_idx)?;
    Ok(())
}

pub fn remove_wire(wire_id: i32) -> i32 {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let wire_map = WIRE_MAP.lock().expect("Lock won't panic");
    // Get wire index
    let idx = if let Some(w) = wire_map.get_by_right(&wire_id) {
        w
    } else {
        return -1;
    };
    // Try to remove
    if netlist.remove_wire(*idx).is_none() {
        -1
    } else {
        0
    }
}

pub fn set_module_position(mod_id: i32, x: f32, y: f32) -> i32 {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let mod_map = MOD_MAP.lock().expect("Lock won't panic");
    // Get mod index from id
    let m = if let Some(m) = mod_map.get_by_right(&mod_id) {
        m
    } else {
        return -1;
    };
    match netlist.set_module_position(*m, x, y) {
        Some(_) => 0,
        None => -1,
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
    let mut wire_map = WIRE_MAP.lock().expect("Lock won't panic");
    let mut pin_map = PIN_MAP.lock().expect("Lock won't panic");

    // Clear all our old drawing state
    (*mod_map).clear();
    (*pin_map).clear();
    (*wire_map).clear();

    // Counter for the ports
    let mut pin_id = 0i32;

    // Grab the modules
    let modules = netlist
        .modules()
        .enumerate()
        .map(|(id, (mi, m))| {
            let id = id as i32;
            mod_map.insert(ModuleIndex(mi), id);
            CModule {
                id,
                position: m.position().clone(),
                name: m.name().to_owned(),
                inputs: m
                    .inputs()
                    .map(|x| {
                        let pin = netlist.get_pin(*x).expect("These will always be valid");
                        let id = pin_id;
                        let name = pin.name().to_owned();
                        // Increment the global id counter
                        pin_id += 1;
                        // Create the lookups
                        pin_map.insert(*x, id);
                        CPort { id, name }
                    })
                    .collect(),
                outputs: m
                    .outputs()
                    .map(|x| {
                        let pin = netlist.get_pin(*x).expect("These will always be valid");
                        let id = pin_id;
                        let name = pin.name().to_owned();
                        // Increment the global id counter
                        pin_id += 1;
                        // Create the lookup
                        pin_map.insert(*x, id);
                        CPort { id, name }
                    })
                    .collect(),
            }
        })
        .collect();
    let wires = netlist
        .wires()
        .enumerate()
        .map(|(id, (idx, (x, y)))| {
            wire_map.insert(WireIndex(idx), id as i32);
            CWire {
                id: id as i32,
                x: *(*pin_map).get_by_left(x).unwrap(),
                y: *(*pin_map).get_by_left(y).unwrap(),
            }
        })
        .collect();
    CGraph { modules, wires }
}

pub fn add_module_from_json_path(path: String, x: f32, y: f32) -> anyhow::Result<()> {
    let mut netlist = NETLIST.lock().expect("Lock won't panic");
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let mi = netlist.add_module_from_json(&buf)?;
    // Set the position
    netlist.set_module_position(mi, x, y);
    Ok(())
}

pub fn get_json_module(mod_id: i32) -> String {
    let netlist = NETLIST.lock().expect("Lock won't panic");
    let mod_map = MOD_MAP.lock().expect("Lock won't panic");
    // Get mod index from id
    let mi = mod_map
        .get_by_right(&mod_id)
        .expect("This module will always exist");
    if let Some(s) = netlist.dump_module_to_json(*mi) {
        s
    } else {
        String::new()
    }
}
