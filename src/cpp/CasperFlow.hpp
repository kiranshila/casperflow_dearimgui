#pragma once

// The GLFW import will bring in system OpenGL bindings
#include <GLFW/glfw3.h>
// ImGui stuff
#include "bindings/imgui_impl_glfw.h"
#include "bindings/imgui_impl_opengl3.h"
#include "imgui.h"
#include <imgui_node_editor.h>
// Rust Stuff
#include "../rust/bindings.h"