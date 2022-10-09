#include "CasperFlow.hpp"

namespace rs = org::cfrs;

int main() {

  GLFWwindow *window = gui_init();

  // Setup some GUI state
  bool first_frame = true;

  ApplicationLog log;
  WindowState ws;

  // Graph
  rs::CGraph graph;

  // Run the gui!
  while (!glfwWindowShouldClose(window) && !ws.quit) {

    // Get the next frame to render to
    gui_newframe();

    // Display the main menu
    draw_main_menu(&ws.show_editor, &ws.show_log, &ws.show_browser,
                   &ws.show_demo, &ws.quit);

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

    if (ImNodes::IsLinkCreated(&ws.start_attr, &ws.stop_attr)) {
      // Attempt to make the connection. Do something? with the error
      try {
        rs::add_wire(ws.start_attr, ws.stop_attr);
        ws.stale_graph = true;
      } catch (std::exception error) {
        log.add_log("%s\n", error.what());
      }
    }

    // Draw the node popup
    if (ImGui::BeginPopup("node_rc_menu")) {
      if (ImGui::MenuItem("Delete module")) {
        rs::remove_module(ws.node);
        ws.stale_graph = true;
      }
      if (ImGui::MenuItem("Get JSON")) {
        auto json = rs::get_json_module(ws.node);
        log.add_log(json.c_str());
      }
      ImGui::EndPopup();
    }

    // Draw the wire popup
    if (ImGui::BeginPopup("wire_rc_menu")) {
      if (ImGui::MenuItem("Delete connection")) {
        if (rs::remove_wire(ws.link) < 0) {
          log.add_log("We tried to delete a link that didn't exist, this "
                      "shouldn't happen.\n");
        } else {
          ws.stale_graph = true;
        }
      }
      ImGui::EndPopup();
    }

    // File selector
    file_selector(&ws.stale_graph);

    // If graph is stale, get a new one
    if (ws.stale_graph) {
      ws.stale_graph = false;
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

    // Render
    gui_render(window);
  }

  // Cleanup everything
  gui_cleanup(window);

  // All done!
  return 0;
}