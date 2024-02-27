find_package(LIBSSH REQUIRED)
set(LIBRARY_NAME "connection")
add_library(${LIBRARY_NAME} "")

if(LIBSSH_FOUND)
    include_directories(${LIBSSH_INCLUDE_DIR})
    link_directories(${LIBSSH_LIBRARY_DIR})
    target_link_libraries(${LIBRARY_NAME} PUBLIC ssh)
else()
    message(WARNING,Unable to find libssh!)
endif()

target_sources(${LIBRARY_NAME}
PRIVATE
    connection.cpp
PUBLIC
    connection.hpp
)




target_include_directories(connection
    PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}
)
