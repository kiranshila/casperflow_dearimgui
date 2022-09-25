#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct PrimitiveHdlNode {
  const char *name;
  unsigned int num_inputs;
  unsigned int num_outputs;
};

extern "C" {

const char *hello_from_rust();

PrimitiveHdlNode *gen_primitive_hdl_node(const char *name,
                                         unsigned int num_inputs,
                                         unsigned int num_outputs);

} // extern "C"
