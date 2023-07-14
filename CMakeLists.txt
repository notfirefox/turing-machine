cmake_minimum_required(VERSION 3.20)
project(turing-machine)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(main)
target_sources(main PRIVATE
./src/impl.c
./src/tape.c
./src/turing.c
./src/main.c
)

