#pragma once

#include "imgui.h"
// The GLFW import will bring in system OpenGL bindings
#include "bindings/imgui_impl_glfw.h"
#include "bindings/imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>

// Setup the GL context and startup ImGui, returning the window
GLFWwindow *init();
// Prepare a new frame for ImGui
void newframe();
// Actually render the frame
void render(GLFWwindow *window);
// Cleanup all the context
void cleanup(GLFWwindow *window);

const ImVec4 clear_color = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);