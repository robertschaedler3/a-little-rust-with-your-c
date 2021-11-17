cmake_minimum_required(VERSION 3.0)

project(c-rust)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

enable_language(Rust)
include(CMakeCargo)

add_subdirectory(lib)

add_subdirectory(src)
