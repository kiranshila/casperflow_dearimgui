#include "CasperFlow.hpp"
#include "imgui.h"
#include "imnodes.h"

namespace rs = org::cfrs;

void cf_editor(bool *p_open, rs::CGraph &graph) {
  ImGui::Begin("Editor", p_open);
  ImNodes::BeginNodeEditor();

  // Draw nodes and wires
  for (auto mod : graph.modules) {
    ImNodes::BeginNode(mod.id);

    // Title
    ImNodes::BeginNodeTitleBar();
    ImGui::TextUnformatted(mod.name.c_str());
    ImNodes::EndNodeTitleBar();

    // Inputs
    for (auto in_port : mod.inputs) {
      ImNodes::BeginInputAttribute(in_port.id);
      ImGui::TextUnformatted(in_port.name.c_str());
      ImNodes::EndInputAttribute();
    }

    // Outputs
    for (auto out_port : mod.outputs) {
      ImNodes::BeginOutputAttribute(out_port.id);
      ImGui::TextUnformatted(out_port.name.c_str());
      ImNodes::EndOutputAttribute();
    }
    ImNodes::EndNode();
  }

  for (auto wire : graph.wires) {
    std::cout << wire.id << std::endl;
    ImNodes::Link(wire.id, wire.x, wire.y);
  }

  ImNodes::MiniMap(0.1f, ImNodesMiniMapLocation_BottomRight);
  ImNodes::EndNodeEditor();
  ImGui::End();
}

void cf_library(bool *p_open) {
  ImGui::Begin("Library Browser", p_open);
  if (ImGui::CollapsingHeader("Primitives")) {
    ImGui::Text("Relational");
    ImGui::Text("Logical");
    ImGui::Text("Delay");
  }
  if (ImGui::CollapsingHeader("IO")) {
    if (ImGui::TreeNode("Networking")) {
      ImGui::Text("10 GbE");
    }
    if (ImGui::TreeNode("GPIO")) {
      ImGui::Text("Software Register");
    }
  }
  ImGui::End();
}

struct ApplicationLog {
  ImGuiTextBuffer buf;
  ImGuiTextFilter filter;
  ImVector<int> offsets;
  bool auto_scroll;

  ApplicationLog() {
    auto_scroll = true;
    clear();
  }

  void clear() {
    buf.clear();
    offsets.clear();
    offsets.push_back(0);
  }

  void add_log(const char *fmt, ...) IM_FMTARGS(2) {
    int old_size = buf.size();
    va_list args;
    va_start(args, fmt);
    buf.appendfv(fmt, args);
    va_end(args);
    for (int new_size = buf.size(); old_size < new_size; old_size++) {
      if (buf[old_size] == '\n') {
        offsets.push_back(old_size + 1);
      }
    }
  }

  void draw(const char *title, bool *p_open = nullptr) {
    if (!ImGui::Begin(title, p_open)) {
      ImGui::End();
      return;
    }
    // Options
    if (ImGui::BeginPopup("Options")) {
      ImGui::Checkbox("Auto-scroll", &auto_scroll);
      ImGui::EndPopup();
    }
    // Main Window
    if (ImGui::Button("Options")) {
      ImGui::OpenPopup("Options");
    }
    ImGui::SameLine();
    bool clr = ImGui::Button("Clear");
    ImGui::SameLine();
    bool copy = ImGui::Button("Copy");
    ImGui::SameLine();
    filter.Draw("Filter", -100.0f);
    ImGui::Separator();
    ImGui::BeginChild("scrolling", ImVec2(0, 0), false,
                      ImGuiWindowFlags_HorizontalScrollbar);
    // Apply button state
    if (clr) {
      clear();
    }
    if (copy) {
      ImGui::LogToClipboard();
    }
    ImGui::PushStyleVar(ImGuiStyleVar_ItemSpacing, ImVec2(0, 0));
    const char *b = buf.begin();
    const char *b_end = buf.end();
    if (filter.IsActive()) {
      for (int line_no = 0; line_no < offsets.Size; line_no++) {
        const char *line_start = b + offsets[line_no];
        const char *line_end = (line_no + 1 < offsets.Size)
                                   ? (b + offsets[line_no + 1] - 1)
                                   : b_end;
        if (filter.PassFilter(line_start, line_end)) {
          ImGui::TextUnformatted(line_start, line_end);
        }
      }
    } else {
      ImGui::TextUnformatted(b, b_end);
    }
    ImGui::PopStyleVar();

    if (auto_scroll && ImGui::GetScrollY() >= ImGui::GetScrollMaxY()) {
      ImGui::SetScrollHereY(1.0f);
    }
    ImGui::EndChild();
    ImGui::End();
  }
};

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

static void cf_main_menu(WindowState *ws) {
  if (ImGui::BeginMainMenuBar()) {
    if (ImGui::BeginMenu("File")) {
      if (ImGui::MenuItem("Open", "CTRL+o")) {
      }
      ImGui::EndMenu();
    }
    if (ImGui::BeginMenu("Edit")) {
      if (ImGui::MenuItem("Undo", "CTRL+z")) {
      }
      ImGui::EndMenu();
    }
    if (ImGui::BeginMenu("View")) {
      ImGui::Checkbox("Editor", &ws->show_editor);
      ImGui::Checkbox("Log", &ws->show_log);
      ImGui::Checkbox("Library Browser", &ws->show_browser);
      ImGui::Checkbox("ImGui Demo", &ws->show_demo);
      ImGui::EndMenu();
    }
    ImGui::EndMainMenuBar();
  }
}

int main() {

  GLFWwindow *window = gui_init();

  // Setup some GUI state
  bool first_frame = true;

  ApplicationLog log;
  WindowState ws;

  // Create a little netlist
  auto mod_idx = rs::add_new_module("Logical");
  rs::add_sized_input_port("A", rs::SizedVerilogKind::Bit, mod_idx, 16);
  rs::add_sized_input_port("B", rs::SizedVerilogKind::Bit, mod_idx, 16);
  rs::add_sized_output_port("Out", rs::SizedVerilogKind::Bit, mod_idx, 16);

  auto mod_idx2 = rs::add_new_module("Hello");
  rs::add_sized_input_port("A", rs::SizedVerilogKind::Bit, mod_idx2, 16);
  rs::add_sized_input_port("B", rs::SizedVerilogKind::Bit, mod_idx2, 16);
  rs::add_sized_output_port("Out", rs::SizedVerilogKind::Bit, mod_idx2, 16);

  rs::connect(mod_idx, 0, mod_idx2, 0);
  rs::connect(mod_idx, 0, mod_idx2, 1);

  // Get graph
  rs::CGraph graph = rs::get_graph();

  // Run the gui!
  while (!glfwWindowShouldClose(window)) {
    // Get the next frame to render to
    gui_newframe();

    // Display the main menu
    cf_main_menu(&ws);

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

    // Run the layout
    if (ws.show_editor)
      cf_editor(&ws.show_editor, graph);
    if (ws.show_browser)
      cf_library(&ws.show_browser);
    if (ws.show_log)
      log.draw("Log", &ws.show_log);
    if (ws.show_demo)
      ImGui::ShowDemoWindow(&ws.show_demo);

    if (ImNodes::IsLinkCreated(&ws.start_attr, &ws.stop_attr)) {
      std::cout << "Try connect " << ws.start_attr << " " << ws.stop_attr
                << std::endl;
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

    if (ImNodes::IsPinHovered(&ws.pin)) {
      ImGui::BeginTooltip();
      ImGui::Text("Pin id: %d", ws.pin);
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