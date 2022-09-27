#pragma once

// ImGui stuff
#include "imgui.h"
#include <imgui_internal.h>
#include <imnodes.h>
// Rust Stuff
#include "lib.rs.h"
// Others
#include "lowlevel_gui.hpp"
#include "ui_components.hpp"
// stdlib
#include <iostream>

// Store the state things that we need to know about every frame that rust
// doesn't need to know about
struct WindowState {
  // UI elements we can view
  bool show_editor;
  bool show_log;
  bool show_browser;
  bool show_demo;
  // Updated when processing new link events
  int start_attr;
  int stop_attr;
  // More link state
  int link;
  int node;
  int pin;
  WindowState() {
    show_editor = true;
    show_log = true;
    show_browser = true;
    show_demo = false;
    start_attr = 0;
    stop_attr = 0;
  }
};