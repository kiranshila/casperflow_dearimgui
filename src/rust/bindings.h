#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class SizedVerilogKind {
  Bit,
  Logic,
};

enum class UnsizedVerilogKind {
  Byte,
  ShortInteger,
  Integer,
  LongInteger,
  Time,
  ShortReal,
  Real,
};

extern "C" {

/// Add a new module with `name` to the global netlist
int32_t add_new_module(const char *name);

/// Add an unnsized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
int32_t add_unsized_input_port(const char *name, int32_t mod_idx, UnsizedVerilogKind kind);

/// Add a sized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
int32_t add_sized_input_port(const char *name,
                             int32_t mod_idx,
                             SizedVerilogKind kind,
                             uintptr_t size);

/// Add an unnsized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
int32_t add_unsized_output_port(const char *name, int32_t mod_idx, UnsizedVerilogKind kind);

/// Add a sized input port with `name` and `kind` to the module indicated by `mod_idx`.
/// Returns -1 if the mod_idx points to an invalid module, otherwise returns the unique port id
int32_t add_sized_output_port(const char *name,
                              int32_t mod_idx,
                              SizedVerilogKind kind,
                              uintptr_t size);

/// Print a debug output of the netlist to stdout
void dump_netlist();

} // extern "C"
