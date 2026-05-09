cmake_minimum_required(VERSION 3.20)
project(driverstation_nt_server VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
    wpilibc
    GIT_REPOSITORY https://github.com/wpilibsuite/allwpilib.git
    GIT_TAG v2026.2.1
    GIT_SHALLOW TRUE
)

set(WITH_JAVA OFF CACHE BOOL "" FORCE)
set(WITH_CSCORE OFF CACHE BOOL "" FORCE)
set(WITH_WPILIB OFF CACHE BOOL "" FORCE)
set(WITH_WPIMATH OFF CACHE BOOL "" FORCE)
set(WITH_WPIUNITS OFF CACHE BOOL "" FORCE)
set(WITH_SIMULATION_MODULES OFF CACHE BOOL "" FORCE)
set(WITH_GUI OFF CACHE BOOL "" FORCE)
set(WITH_TESTS OFF CACHE BOOL "" FORCE)
set(WITH_EXAMPLES OFF CACHE BOOL "" FORCE)
set(WITH_NTCORE ON  CACHE BOOL "" FORCE)

FetchContent_MakeAvailable(wpilibc)

add_executable(driverstation_nt_server
        src/main/cpp/main.cpp
        src/main/cpp/NTClient.cpp
        src/main/cpp/SerialPort.cpp
        src/main/cpp/PatternResolver.cpp
        src/main/cpp/LarsonAnimator.cpp
)

target_include_directories(driverstation_nt_server
    PRIVATE
    src/main/include          # ← your headers
    "${ntcore_headers_SOURCE_DIR}"
    "${wpiutil_headers_SOURCE_DIR}"
)

target_link_libraries(driverstation_nt_server
    PRIVATE
    ntcore
    wpiutil
)

# Windows serial needs these
if(WIN32)
    target_link_libraries(driverstation_nt_server PRIVATE setupapi)
endif()
