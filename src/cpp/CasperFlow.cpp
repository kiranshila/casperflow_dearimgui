#include "CasperFlow.hpp"
#include "imnodes.h"

void in_port(int node_id, int pin_id) {
  ImNodes::BeginNode(node_id);
  ImNodes::BeginOutputAttribute(pin_id);
  ImGui::Text("In");
  ImNodes::EndOutputAttribute();
  ImNodes::EndNode();
}

void out_port(int node_id, int pin_id) {
  ImNodes::BeginNode(node_id);
  ImNodes::BeginInputAttribute(pin_id);
  ImGui::Text("Out");
  ImNodes::EndInputAttribute();
  ImNodes::EndNode();
}

void logical(int node_id, int in_a_id, int in_b_id, int out_id) {
  ImNodes::BeginNode(node_id);

  ImNodes::BeginNodeTitleBar();
  ImGui::TextUnformatted("Logical");
  ImNodes::EndNodeTitleBar();

  ImNodes::BeginInputAttribute(in_a_id);
  ImGui::Text("A");
  ImNodes::EndInputAttribute();

  ImNodes::BeginInputAttribute(in_b_id);
  ImGui::Text("B");
  ImNodes::EndInputAttribute();

  ImNodes::BeginOutputAttribute(out_id);
  ImGui::Text("Out");
  ImNodes::EndOutputAttribute();

  ImNodes::EndNode();
}

void cf_editor(bool *p_open) {
  ImGui::Begin("Editor", p_open);
  ImNodes::BeginNodeEditor();

  in_port(1, 2);
  in_port(3, 4);
  in_port(5, 6);

  logical(7, 8, 9, 10);
  logical(11, 12, 13, 14);

  out_port(15, 16);

  ImNodes::Link(17, 2, 8);
  ImNodes::Link(18, 4, 9);
  ImNodes::Link(19, 6, 12);
  ImNodes::Link(20, 10, 13);
  ImNodes::Link(21, 14, 16);

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
  bool show_editor;
  bool show_log;
  bool show_browser;
  bool show_demo;
  WindowState() {
    show_editor = true;
    show_log = true;
    show_browser = true;
    show_demo = false;
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
      cf_editor(&ws.show_editor);
    if (ws.show_browser)
      cf_library(&ws.show_browser);
    if (ws.show_log)
      log.draw("Log", &ws.show_log);
    if (ws.show_demo)
      ImGui::ShowDemoWindow(&ws.show_demo);

    // Render
    gui_render(window);
  }

  // Cleanup everything
  gui_cleanup(window);

  // All done!
  return 0;
}