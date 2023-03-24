cmake_minimum_required(VERSION 3.21)
project(Matrix_Aufgabe C)

set(CMAKE_C_STANDARD 17)

add_executable(Matrix_Aufgabe main.c matrix/matrix.h matrix/matrix.c)
