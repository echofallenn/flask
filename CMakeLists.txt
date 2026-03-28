cmake_minimum_required(VERSION 3.16)
project(flask_resolver CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(resolver STATIC
    cpp/resolver.cpp
)

target_include_directories(resolver PUBLIC cpp/)

set_target_properties(resolver PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_SOURCE_DIR}/target/cxx"
    POSITION_INDEPENDENT_CODE ON
)
