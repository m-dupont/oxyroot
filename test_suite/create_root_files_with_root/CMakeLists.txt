# CMakeLists.txt for event package. It creates a library with dictionary and a main program
cmake_minimum_required(VERSION 3.9 FATAL_ERROR)

project(customTreeExample)

# You need to tell CMake where to find the ROOT installation. This can be done in a number of ways:
#   - ROOT built with classic configure/make use the provided $ROOTSYS/etc/cmake/FindROOT.cmake
#   - ROOT built with CMake. Add in CMAKE_PREFIX_PATH the installation prefix for ROOT
list(APPEND CMAKE_PREFIX_PATH $ENV{ROOTSYS})

#---Locate the ROOT package and defines a number of variables (e.g. ROOT_INCLUDE_DIRS)
find_package(ROOT REQUIRED COMPONENTS MathCore RIO Hist Tree Net Graf3d Graf Gpad Thread TreePlayer)

#---Define useful ROOT functions and macros (e.g. ROOT_GENERATE_DICTIONARY)
include(${ROOT_USE_FILE})

#---Add include directory of ROOT to the build
include_directories(${CMAKE_SOURCE_DIR} ${ROOT_INCLUDE_DIRS})

#---Add library directory of ROOT to the build
link_directories(${ROOT_LIBRARY_DIR})

#---Set Flags
add_definitions(${ROOT_CXX_FLAGS}  )

# CMake function provided by ROOT, used to generate the dictionary file, G__data2Tree.cxx
#     See this link for further details:
#     https://root.cern/manual/io_custom_classes/#using-cmake
ROOT_GENERATE_DICTIONARY(G__data2Tree data2Tree.hpp LINKDEF data2TreeLinkDef.hpp)

#---Create a shared library from
#   * the previously generated dictionary, G__data2Tree.cxx
#   * the class implementation
add_library(data2TreeLib SHARED G__data2Tree.cxx)
target_link_libraries(data2TreeLib ${ROOT_LIBRARIES}  ) 

#---Create  a main program using the library
add_executable(treeExample main.cpp)
target_link_libraries(treeExample ${ROOT_LIBRARIES} data2TreeLib)

# Enable PIC
set_property(TARGET treeExample PROPERTY POSITION_INDEPENDENT_CODE ON)
