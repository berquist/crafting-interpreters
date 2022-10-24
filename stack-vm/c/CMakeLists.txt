cmake_minimum_required(VERSION 3.12)

project(CLOX C)

set(SRC
    src/chunk.c
    src/debug.c
    src/main.c
    src/memory.c
    src/value.c
    src/vm.c
)

add_executable(clox ${SRC})
add_compile_definitions(DEBUG_TRACE_EXECUTION)
