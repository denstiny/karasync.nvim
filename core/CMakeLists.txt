######################################################################
# @author      : denstiny (2254228017@qq.com)
# @file        : CMakeLists
# @created     : 星期二 2月 27, 2024 13:41:35 CST
######################################################################

cmake_minimum_required(VERSION 3.20)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
project(karasync CXX)
set(CMAKE_CXX_STANDARD 11)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/bin)
set(CMAKE_LIBRRAY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/bin)

add_subdirectory(src)
