cmake_minimum_required(VERSION 3.24)

project(CLOX C)

set(SRC
    src/chunk.c
    src/debug.c
    src/main.c
    src/memory.c
    src/value.c
)

add_executable(clox ${SRC})
