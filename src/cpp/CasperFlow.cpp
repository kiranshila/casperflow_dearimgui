#include "CasperFlow.hpp"

namespace ed = ax::NodeEditor;

static void main_menu() {
  if (ImGui::BeginMainMenuBar()) {
    if (ImGui::BeginMenu("File")) {
      if (ImGui::MenuItem("Open", "CTRL+O")) {
      }
      ImGui::EndMenu();
    }
    if (ImGui::BeginMenu("Edit")) {
      if (ImGui::MenuItem("Undo", "CTRL+Z")) {
      }
      ImGui::EndMenu();
    }
    ImGui::EndMainMenuBar();
  }
}

// The annoying thing is that you can't dock the node editor itself into other
// windows, but you can do it the other way around. So, we're going to make the
// node editor the primary window  and dock the toolbars etc into that.
static void node_editor(ed::EditorContext *ctx) {
  // Make the thing full screen
  static ImGuiWindowFlags flags =
      ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoMove |
      ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoSavedSettings;
  const ImGuiViewport *viewport = ImGui::GetMainViewport();
  ImGui::SetNextWindowPos(viewport->WorkPos);
  ImGui::SetNextWindowSize(viewport->WorkSize);
  // Now draw the node editor
  ImGui::Begin("Graph Editor", NULL, flags);
  ed::SetCurrentEditor(ctx);
  ed::Begin("My Editor", ImVec2(0.0, 0.0f));
  int uniqueId = 1;
  // Start drawing nodes.
  ed::BeginNode(uniqueId++);
  ImGui::Text("Node A");
  ed::BeginPin(uniqueId++, ed::PinKind::Input);
  ImGui::Text("In");
  ed::EndPin();
  ImGui::SameLine();
  ed::BeginPin(uniqueId++, ed::PinKind::Output);
  ImGui::Text("Out");
  ed::EndPin();
  ed::EndNode();
  ed::End();
  ed::SetCurrentEditor(nullptr);
  ImGui::End();
}

int main() {
  // Try out rust!
  hello_from_rust();

  // Setup window
  if (!glfwInit()) {
    return 1;
  }

  // Specify the OpenGL and GLSL version we're targeting
  const char *glsl_version = "#version 330";
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);

  // Core profile turns off the backwards compat stuff we probably don't want
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  // Create window with graphics context
  GLFWwindow *window = glfwCreateWindow(1280, 720, "CasperFlow", NULL, NULL);

  // Check if we actually made the window
  if (window == NULL) {
    return 1;
  }

  // Make current and enable vsync
  glfwMakeContextCurrent(window);
  glfwSwapInterval(1);

  // Initialize imgui
  IMGUI_CHECKVERSION();
  ImGui::CreateContext();
  ImGuiIO &io = ImGui::GetIO();

  // Hook up imgui to the GLFW window
  ImGui_ImplGlfw_InitForOpenGL(window, true);
  // Hook up the renderer
  ImGui_ImplOpenGL3_Init(glsl_version);
  // Dark mode only for now
  ImGui::StyleColorsDark();

  // Some program state
  ImVec4 clear_color = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);
  ed::EditorContext *ctx = ed::CreateEditor();

  // Run the gui!
  while (!glfwWindowShouldClose(window)) {
    // Update the state of events that have happened to the window
    glfwPollEvents();

    // Setup the imgui frame
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();

    // Display the main menu
    main_menu();

    // Node editor
    node_editor(ctx);

    // ImGui::ShowDemoWindow();

    // Render
    ImGui::Render();
    int display_w, display_h;
    glfwGetFramebufferSize(window, &display_w, &display_h);
    glViewport(0, 0, display_w, display_h);
    glClearColor(clear_color.x * clear_color.w, clear_color.y * clear_color.w,
                 clear_color.z * clear_color.w, clear_color.w);
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());

    glfwSwapBuffers(window);
  }

  // Cleanup
  ImGui_ImplOpenGL3_Shutdown();
  ImGui_ImplGlfw_Shutdown();
  ImGui::DestroyContext();

  glfwDestroyWindow(window);
  glfwTerminate();

  // All done!
  return 0;
}