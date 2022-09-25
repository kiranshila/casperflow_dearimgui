add_executable(casper_flow 
    CasperFlow.cpp
    lowlevel_gui.cpp
    node_style.cpp
    bindings/imgui_impl_glfw.cpp
    bindings/imgui_impl_opengl3.cpp)

target_link_libraries(casper_flow ${CONAN_LIBS} imnodes casper_flow_backend)