pub mod verilog;

use generational_arena::Arena;
use verilog::Module;

static mut NETLIST: Netlist = Netlist { modules: vec![] };

struct Netlist {
    modules: Vec<Module>,
}
