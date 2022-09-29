#include "ui_components.hpp"
#include <iostream>

/// Draw the graph and wires in a window called "Editor"
void draw_editor(bool *p_open, org::cfrs::CGraph &graph) {
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
    ImNodes::Link(wire.id, wire.x, wire.y);
  }

  ImNodes::MiniMap(0.1f, ImNodesMiniMapLocation_BottomRight);
  ImNodes::EndNodeEditor();
  ImGui::End();
}

// Draw the library browser
void draw_library(bool *p_open) {
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

// Draw the main menu - file, window, etc.
void draw_main_menu(bool *editor_open, bool *log_open, bool *browser_open,
                    bool *demo_open) {
  if (ImGui::BeginMainMenuBar()) {
    if (ImGui::BeginMenu("File")) {
      if (ImGui::MenuItem("Open library", "CTRL+o")) {
        ImGuiFileDialog::Instance()->OpenDialog("ChooseLibDlgKey",
                                                "Choose File", ".json", ".");
      }
      ImGui::EndMenu();
    }
    if (ImGui::BeginMenu("Edit")) {
      if (ImGui::MenuItem("Dump netlist", "CTRL+d")) {
        org::cfrs::dump_netlist();
      }
      ImGui::EndMenu();
    }
    if (ImGui::BeginMenu("View")) {
      ImGui::Checkbox("Editor", editor_open);
      ImGui::Checkbox("Log", log_open);
      ImGui::Checkbox("Library Browser", browser_open);
      ImGui::Checkbox("ImGui Demo", demo_open);
      ImGui::EndMenu();
    }
    ImGui::EndMainMenuBar();
  }
}

/// Logging - taken from the ImGui Demo

ApplicationLog::ApplicationLog() {
  auto_scroll = true;
  clear();
}

void ApplicationLog::clear() {
  buf.clear();
  offsets.clear();
  offsets.push_back(0);
}

void ApplicationLog::add_log(const char *fmt, ...) {
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

void ApplicationLog::draw(const char *title, bool *p_open = nullptr) {
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
      const char *line_end =
          (line_no + 1 < offsets.Size) ? (b + offsets[line_no + 1] - 1) : b_end;
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

bool file_selector() {
  bool stale = false;
  // display
  if (ImGuiFileDialog::Instance()->Display("ChooseLibDlgKey")) {
    // action if OK
    if (ImGuiFileDialog::Instance()->IsOk()) {
      std::string path = ImGuiFileDialog::Instance()->GetFilePathName();
      // action
      org::cfrs::add_module_from_json_path(path);
      stale = true;
    }
    // close
    ImGuiFileDialog::Instance()->Close();
  }
  return stale;
}