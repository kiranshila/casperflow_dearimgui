# Imgui
set(IMGUI_DIR ${CMAKE_CURRENT_SOURCE_DIR}/imgui)
add_library(imgui STATIC)

target_sources(imgui
    PRIVATE
    ${IMGUI_DIR}/imgui_demo.cpp
    ${IMGUI_DIR}/imgui_draw.cpp
    ${IMGUI_DIR}/imgui_tables.cpp
    ${IMGUI_DIR}/imgui_widgets.cpp
    ${IMGUI_DIR}/imgui.cpp
    PRIVATE
    ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp)

target_include_directories(imgui
    PUBLIC ${IMGUI_DIR}
    PUBLIC ${IMGUI_DIR}/backends)

# Imnodes
add_library(imnodes)
target_sources(imnodes PRIVATE imnodes/imnodes.cpp)
target_include_directories(imnodes PUBLIC imnodes)
target_link_libraries(imnodes PUBLIC imgui)

# ImGuiFileDialog
add_library(imgui_file_dialog)
target_sources(imgui_file_dialog PRIVATE ImGuiFileDialog/ImGuiFileDialog.cpp)
target_include_directories(imgui_file_dialog PUBLIC ImGuiFileDialog)
target_link_libraries(imgui_file_dialog PUBLIC imgui)