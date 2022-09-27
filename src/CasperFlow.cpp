#include "CasperFlow.hpp"

namespace rs = org::cfrs;

int main() {

  GLFWwindow *window = gui_init();

  // Setup some GUI state
  bool first_frame = true;

  ApplicationLog log;
  WindowState ws;

  // Create a little netlist
  auto mod_idx = rs::add_new_module("Logical");
  rs::add_sized_input_port("A", rs::SizedVerilogKind::Reg, mod_idx, 16, false);
  rs::add_sized_input_port("B", rs::SizedVerilogKind::Reg, mod_idx, 16, false);
  rs::add_sized_output_port("Out", rs::SizedVerilogKind::Reg, mod_idx, 16,
                            false);

  auto mod_idx2 = rs::add_new_module("Hello");
  rs::add_sized_input_port("A", rs::SizedVerilogKind::Supply1, mod_idx2, 16,
                           false);
  rs::add_sized_input_port("B", rs::SizedVerilogKind::Reg, mod_idx2, 16, false);
  rs::add_sized_output_port("Out", rs::SizedVerilogKind::Reg, mod_idx2, 16,
                            false);

  auto mod_idx3 = rs::add_new_module("Ivy");
  rs::add_sized_input_port("A", rs::SizedVerilogKind::Reg, mod_idx3, 16, false);
  rs::add_sized_input_port("B", rs::SizedVerilogKind::Reg, mod_idx3, 16, false);
  rs::add_sized_output_port("Out", rs::SizedVerilogKind::Reg, mod_idx3, 16,
                            false);

  // Track to see if the graph is stale
  bool stale = true;

  // Graph
  rs::CGraph graph;

  // Run the gui!
  while (!glfwWindowShouldClose(window)) {
    // Get the next frame to render to
    gui_newframe();

    // Display the main menu
    draw_main_menu(&ws.show_editor, &ws.show_log, &ws.show_browser,
                   &ws.show_demo);

    // Create central dockspace
    auto ds_id = ImGui::DockSpaceOverViewport(ImGui::GetMainViewport());

    // Things to do on the first frame
    if (first_frame) {
      first_frame = false;
      // Create initial layout
      auto dock_id_bot = ImGui::DockBuilderSplitNode(ds_id, ImGuiDir_Down,
                                                     0.20f, nullptr, &ds_id);
      auto dock_id_left = ImGui::DockBuilderSplitNode(ds_id, ImGuiDir_Left,
                                                      0.20f, nullptr, &ds_id);
      ImGui::DockBuilderDockWindow("Log", dock_id_bot);
      ImGui::DockBuilderDockWindow("Library Browser", dock_id_left);
      ImGui::DockBuilderDockWindow("Editor", ds_id);
      ImGui::DockBuilderFinish(ds_id);
    }

    if (stale) {
      stale = false;
      graph = rs::get_graph();
    }

    // Run the layout
    if (ws.show_editor)
      draw_editor(&ws.show_editor, graph);
    if (ws.show_browser)
      draw_library(&ws.show_browser);
    if (ws.show_log)
      log.draw("Log", &ws.show_log);
    if (ws.show_demo)
      ImGui::ShowDemoWindow(&ws.show_demo);

    // Check if we right clicked a node
    bool open_node_popup =
        ImGui::IsWindowFocused(ImGuiFocusedFlags_RootAndChildWindows) &&
        ImNodes::IsEditorHovered() && ImGui::IsMouseReleased(1) &&
        !ImGui::IsMouseDragging(1);

    if (ImNodes::IsLinkHovered(&ws.link)) {
      ImGui::BeginTooltip();
      ImGui::Text("Link id: %d", ws.link);
      ImGui::EndTooltip();

      if (ImGui::IsMouseReleased(1) && !ImGui::IsMouseDragging(1)) {
        ImGui::OpenPopup("wire_rc_menu");
      }
    }

    if (ImNodes::IsNodeHovered(&ws.node)) {
      ImGui::BeginTooltip();
      ImGui::Text("Node id: %d", ws.node);
      ImGui::EndTooltip();

      // If right click on node, open the node context menu
      if (ImGui::IsMouseReleased(1) && !ImGui::IsMouseDragging(1)) {
        ImGui::OpenPopup("node_rc_menu");
      }
    }

    if (ImNodes::IsPinHovered(&ws.pin)) {
      ImGui::BeginTooltip();
      ImGui::Text("Pin type: %s", rs::get_type(ws.pin).c_str());
      ImGui::EndTooltip();
    }

    if (ImNodes::IsLinkCreated(&ws.start_attr, &ws.stop_attr)) {
      // Attempt to make the connection. Do something? with the error
      auto result = rs::connect2(ws.start_attr, ws.stop_attr);
      switch (result) {
      case org::cfrs::ConnectionResult::BadIndex:
        log.add_log("We somehow got a bad pin or module index, this shouldn't "
                    "happen\n");
        break;
      case org::cfrs::ConnectionResult::DirectionMismatch:
        log.add_log("Inputs must be connected to outputs\n");
        break;
      case org::cfrs::ConnectionResult::TypeMismatch:
        log.add_log("The port types disagree, check the port types on either "
                    "side of the connection\n");
        break;
      case org::cfrs::ConnectionResult::InputDriven:
        log.add_log(
            "Input is already driven, delete the existing connection\n");
        break;
      case org::cfrs::ConnectionResult::ConnectionOk:
        stale = true;
        break;
      }
    }

    // Draw the node popup
    if (ImGui::BeginPopup("node_rc_menu")) {
      if (ImGui::MenuItem("Delete module")) {
        rs::delete_module(ws.node);
        stale = true;
      }
      ImGui::EndPopup();
    }

    // Draw the wire popup
    if (ImGui::BeginPopup("wire_rc_menu")) {
      if (ImGui::MenuItem("Delete connection")) {
        if (rs ::delete_wire(ws.link) < 0) {
          log.add_log("We tried to delete a link that didn't exist, this "
                      "shouldn't happen.\n");
        } else {
          stale = true;
        }
      }
      ImGui::EndPopup();
    }

    // Render
    gui_render(window);
  }

  // Cleanup everything
  gui_cleanup(window);

  // All done!
  return 0;
}