# CasperFlow

CasperFlow is a graphical hardware description language for the CASPER Collaboration.

## Goals
- Be able to design *relatively simple* gateware in a familiar Simulink-like GUI
- Track the model netlist in a fully documented netlist file that makes for easy version control
- Support heiarchial design
- Typesafe on connections, no waiting for Ctrl-D to flood your screen with errors
- Emit verilog + constraints file for use in either the FOSS F4PGA backends or Vivado
- Easy importing of foreign verilog (Drag in a .v maybe?)
- Performant
- Tested

## Stretch Goals
- Baked in simulation support with verilator
- Support some scripting for higher level blocks (mlua + LuaJIT maybe?)
- Replace some CASPER-yellow blocks with FOSS IP (10 GbE, ADC, MicroBlaze -> RISCV, etc.)
- Support other HDLs like VHDL, SystemVerilog, that rust one
- WebAssembly support?

## Structure

All the GUI stuff is written in CPP for now, until the ImGui wrapper matures
We're using Conan to grab the CPP deps, and using Corrosion to build and link the rust stuff.