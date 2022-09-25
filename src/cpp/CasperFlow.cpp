#include "CasperFlow.hpp"
#include "imgui.h"
#include "imgui_node_editor.h"

namespace ed = ax::NodeEditor;

void cf_editor(ed::EditorContext *ctx, bool *p_open) {
  ImGui::Begin("Editor", p_open);
  ed::SetCurrentEditor(ctx);
  ax::NodeEditor::Begin("Editor");
  ax::NodeEditor::End();
  ed::SetCurrentEditor(nullptr);
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
  WindowState() {
    show_editor = true;
    show_log = true;
    show_browser = true;
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
      ImGui::EndMenu();
    }
    ImGui::EndMainMenuBar();
  }
}

int main() {

  GLFWwindow *window = init();

  // Setup some GUI state
  bool first_frame = true;
  ed::EditorContext *ed_ctx = ed::CreateEditor(nullptr);
  ApplicationLog log;
  WindowState ws;

  log.add_log("%s", hello_from_rust());

  // Run the gui!
  while (!glfwWindowShouldClose(window)) {
    // Get the next frame to render to
    newframe();

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

    if (ws.show_editor)
      cf_editor(ed_ctx, &ws.show_editor);
    if (ws.show_browser)
      cf_library(&ws.show_browser);
    if (ws.show_log)
      log.draw("Log", &ws.show_log);

    // ImGui::ShowDemoWindow();

    // Render
    render(window);
  }

  // Cleanup everything
  cleanup(window);
  ed::DestroyEditor(ed_ctx);

  // All done!
  return 0;
}