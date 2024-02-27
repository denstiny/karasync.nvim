add_executable(${PROJECT_NAME} main.cpp)

add_subdirectory(connection)

target_link_libraries(${PROJECT_NAME}
PRIVATE
    connection
)
