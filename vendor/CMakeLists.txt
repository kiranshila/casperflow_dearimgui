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