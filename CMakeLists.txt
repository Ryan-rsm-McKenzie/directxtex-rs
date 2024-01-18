cmake_minimum_required(VERSION 3.27)
set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)

project(directxtex-ffi LANGUAGES CXX)

if("${CMAKE_CXX_COMPILER_ID}" MATCHES "Clang|GNU")
	set(CMAKE_CXX_EXTENSIONS OFF)
endif()

set(CMAKE_CXX_STANDARD_REQUIRED ON)

if("${PROJECT_SOURCE_DIR}" STREQUAL "${PROJECT_BINARY_DIR}")
	message(FATAL_ERROR "in-source builds are not allowed")
endif()

add_subdirectory("external/DirectXMath" "DirectXMath")

set(DXHEADERS_BUILD_TEST OFF)
set(DXHEADERS_INSTALL OFF)
set(DXHEADERS_BUILD_GOOGLE_TEST OFF)
add_subdirectory("external/DirectX-Headers" "DirectX-Headers")

set(BUILD_TOOLS OFF)
set(BUILD_SAMPLE OFF)
set(BUILD_DX11 OFF)
set(BUILD_DX12 OFF)
set(BC_USE_OPENMP OFF)
add_subdirectory("external/DirectXTex" "DirectXTex")

add_subdirectory("ffi")
