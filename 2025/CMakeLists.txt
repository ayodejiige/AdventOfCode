cmake_minimum_required(VERSION 3.16)
project(AoC2025)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add submodules
add_subdirectory(external/CLI11)

# Include directories
include_directories(include)

# Find all day source files
file(GLOB SOURCES "src/*.cpp")

# Create executable
add_executable(aoc2025 ${SOURCES})

# Link CLI11
target_link_libraries(aoc2025 PRIVATE CLI11::CLI11)

# Print build information
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "Compiler: ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")