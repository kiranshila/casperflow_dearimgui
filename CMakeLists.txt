cmake_minimum_required(VERSION 3.15)
project("CasperFlow" CXX)

# Include all the stuff from conan
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

# Include corrosion for rust builds
find_package(Corrosion REQUIRED)

add_subdirectory("vendor")
add_subdirectory("src")