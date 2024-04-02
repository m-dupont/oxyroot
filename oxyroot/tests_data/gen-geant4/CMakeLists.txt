cmake_minimum_required(VERSION 3.17)
project(gen_geant4)

set(CMAKE_CXX_STANDARD 17)

find_package(Geant4 REQUIRED)

#----------------------------------------------------------------------------
# Setup Geant4 include directories and compile definitions
#
include(${Geant4_USE_FILE})

include_directories(        ${Geant4_INCLUDE_DIR})

add_executable(gen_geant4 main.cpp)
target_link_libraries(gen_geant4 ${Geant4_LIBRARIES} )

