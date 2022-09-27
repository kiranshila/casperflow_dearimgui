cmake_minimum_required(VERSION 3.22)
project("casper_flow" CXX)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_BINARY_DIR}/target)

# Build the correct release type
if(CMAKE_BUILD_TYPE STREQUAL Debug)
    set(CARGO_PROFILE dev)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/debug)
elseif(CMAKE_BUILD_TYPE STREQUAL RelWithDebInfo)
    set(CARGO_PROFILE release-with-debug)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release-with-debug)
else()
    set(CARGO_PROFILE release)
    set(CARGO_BUILD_FOLDER ${CARGO_TARGET_DIR}/release)
endif()

# All the rust artifact stuff
set(CFRS_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(CFRS_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/cfrs/src/lib.rs.cc)
set(CFRS_LIB ${CARGO_BUILD_FOLDER}/${CMAKE_STATIC_LIBRARY_PREFIX}cfrs${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${CFRS_BRIDGE_CPP} ${CFRS_LIB}
        COMMAND CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build --profile ${CARGO_PROFILE} --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${CFRS_SOURCE_FILE}
        USES_TERMINAL
)

# Include all the stuff from conan
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# Add the imnodes deps
add_subdirectory("vendor")

add_executable(${PROJECT_NAME} 
    ${CMAKE_SOURCE_DIR}/src/CasperFlow.cpp
    ${CMAKE_SOURCE_DIR}/src/lowlevel_gui.cpp
    ${CMAKE_SOURCE_DIR}/src/ui_components.cpp
    ${CMAKE_SOURCE_DIR}/src/bindings/imgui_impl_glfw.cpp
    ${CMAKE_SOURCE_DIR}/src/bindings/imgui_impl_opengl3.cpp
    ${CFRS_BRIDGE_CPP})

target_include_directories(
    ${PROJECT_NAME} 
    PRIVATE
    ${CARGO_TARGET_DIR}/cxxbridge/cfrs/src/)

target_link_libraries(${PROJECT_NAME} ${CONAN_LIBS} imnodes imgui_file_dialog ${CFRS_LIB})