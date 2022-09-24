add_executable(casper_flow CasperFlow.cpp bindings/imgui_impl_glfw.cpp bindings/imgui_impl_opengl3.cpp)

target_link_libraries(casper_flow ${CONAN_LIBS} imgui_node_editor casper_flow_backend)