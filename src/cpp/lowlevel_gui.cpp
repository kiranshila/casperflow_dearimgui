#include "lowlevel_gui.hpp"

GLFWwindow *init() {
  // Setup window
  if (!glfwInit()) {
    return nullptr;
  }
  // Specify the OpenGL and GLSL version we're targeting
  const char *glsl_version = "#version 330";
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);

  // Core profile turns off the backwards compat stuff we probably don't want
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  // Create window with graphics context
  GLFWwindow *window =
      glfwCreateWindow(1280, 720, "CasperFlow", nullptr, nullptr);

  // Check if we actually made the window
  if (window == nullptr) {
    return nullptr;
  }

  // Make current and enable vsync
  glfwMakeContextCurrent(window);
  glfwSwapInterval(1);

  // Initialize imgui
  IMGUI_CHECKVERSION();
  ImGui::CreateContext();
  ImGuiIO &io = ImGui::GetIO();
  io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;

  // No state pls
  io.IniFilename = nullptr;

  // Hook up imgui to the GLFW window
  ImGui_ImplGlfw_InitForOpenGL(window, true);
  // Hook up the renderer
  ImGui_ImplOpenGL3_Init(glsl_version);
  // Dark mode only for now
  ImGui::StyleColorsDark();

  return window;
}

void newframe() {
  // Update the state of events that have happened to the window
  glfwPollEvents();

  // Setup the imgui frame
  ImGui_ImplOpenGL3_NewFrame();
  ImGui_ImplGlfw_NewFrame();
  ImGui::NewFrame();
}

void render(GLFWwindow *window) {
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

void cleanup(GLFWwindow *window) {
  ImGui_ImplOpenGL3_Shutdown();
  ImGui_ImplGlfw_Shutdown();
  ImGui::DestroyContext();
  glfwDestroyWindow(window);
  glfwTerminate();
}