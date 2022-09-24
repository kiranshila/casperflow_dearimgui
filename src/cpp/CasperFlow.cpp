#include "CasperFlow.hpp"

static void main_menu() {
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
    ImGui::EndMainMenuBar();
  }
}

int main() {
  // Try out rust!
  hello_from_rust();

  GLFWwindow *window = init();

  // Run the gui!
  while (!glfwWindowShouldClose(window)) {
    // Get the next frame to render to
    newframe();

    // Display the main menu
    main_menu();

    // Render
    render(window);
  }

  // All done!
  return 0;
}