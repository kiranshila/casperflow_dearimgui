# Imnodes
add_library(imnodes)
target_sources(imnodes PRIVATE
    imnodes/imnodes.h
    imnodes/imnodes_internal.h
    imnodes/imnodes.cpp)
target_include_directories(imnodes PUBLIC imnodes)
target_link_libraries(imnodes PUBLIC imgui)