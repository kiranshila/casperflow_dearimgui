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

    // Draw the right click menu for the editor
    if (ImGui::BeginPopup("rc_menu")) {

      // Collect all the highlighted things (nodes/wires)
      int num_nodes = ImNodes::NumSelectedNodes();
      int num_links = ImNodes::NumSelectedLinks();

      bool delete_single_link = (ws.link != -1) && (num_nodes == 0) && (num_links == 0);
      bool delete_single_node = (ws.node != -1) && (num_nodes == 0) && (num_links == 0);
      bool bulk_delete = (num_nodes >= 0) || (num_links >= 0);

      if (bulk_delete){
        if (ImGui::MenuItem("Delete all")) {
        // Oh god, heap
        int* node_ids = (int*)malloc(sizeof(int)*num_nodes);
        int* link_ids = (int*)malloc(sizeof(int)*num_links);
        ImNodes::GetSelectedNodes(node_ids);
        ImNodes::GetSelectedLinks(link_ids);

        for (int i = 0; i < num_links; i++) {
          if (rs::remove_wire(link_ids[i]) < 0) {
            log.add_log("We tried to delete a link that didn't exist, this "
                        "shouldn't happen.\n");
          }
        }

        for (int i = 0; i < num_nodes; i++) {
          rs::remove_module(node_ids[i]);
        }

        // Don't forget you're programming c, you big dummy
        free(node_ids);
        free(link_ids);

        ws.stale_graph = true;
        }
      }

      else if (delete_single_link) {
        if (ImGui::MenuItem("Delete wire")) {
        if (rs::remove_wire(ws.link) < 0) {
          log.add_log("We tried to delete a link that didn't exist, this "
                      "shouldn't happen.\n");
        }
        ws.stale_graph = true;
        }
      }

      else if (delete_single_node) {
        if (ImGui::MenuItem("Delete node")) {
        rs::remove_module(ws.node);
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

    // Process any right click action as I don't think there is a way yet
    // to discriminate between docked windows
    if (ImGui::IsMouseReleased(1) && !ImGui::IsMouseDragging(1)) {
      ImGui::OpenPopup("rc_menu");
    }

    if (ImNodes::IsLinkHovered(&ws.link)) {
      ImGui::BeginTooltip();
      ImGui::Text("Link id: %d", ws.link);
      ImGui::EndTooltip();
    }

    if (ImNodes::IsNodeHovered(&ws.node)) {
      ImGui::BeginTooltip();
      ImGui::Text("Node id: %d", ws.node);
      ImGui::EndTooltip();
    }

    // Render
    gui_render(window);
  }

  // Cleanup everything
  gui_cleanup(window);

  // All done!
  return 0;
}