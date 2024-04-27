cmake_minimum_required(VERSION 3.26)
project(mod_template)

set(CMAKE_CXX_STANDARD 20)

add_library(${PROJECT_NAME} SHARED core.cpp)

target_link_libraries(${PROJECT_NAME} android log)
