#include "node_style.hpp"
#include "imgui_internal.h"
#include <algorithm>

namespace ed = ax::NodeEditor;

void draw_icon(ImDrawList *draw_list, const ImVec2 &a, const ImVec2 &b,
               bool filled, ImU32 color, ImU32 inner_color) {
  auto rect = ImRect(a, b);
  auto rect_x = rect.Min.x;
  auto rect_y = rect.Min.y;
  auto rect_w = rect.Max.x - rect.Min.x;
  auto rect_h = rect.Max.y - rect.Min.y;
  auto rect_center_x = (rect.Min.x + rect.Max.x) * 0.5f;
  auto rect_center_y = (rect.Min.y + rect.Max.y) * 0.5f;
  auto rect_center = ImVec2(rect_center_x, rect_center_y);
  const auto outline_scale = rect_w / 24.0f;
  const auto extra_segments =
      static_cast<int>(2 * outline_scale); // for full circle
}

void draw_primitive_node() {
  ed::BeginNode(1);
  ImGui::Text("Logical");

  ed::BeginPin(2, ed::PinKind::Input);
  ImGui::Text("-> In");
  ed::EndPin();

  ed::BeginPin(3, ed::PinKind::Output);
  ImGui::Text("-> In");
  ed::EndPin();

  ed::EndNode();
}