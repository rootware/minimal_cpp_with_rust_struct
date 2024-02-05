cmake_minimum_required(VERSION 3.22)
project(cpp_with_rust)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(ENVIRONMENT_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(ENVIRONMENT_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/environment/src/lib.rs.cc)
set(ENVIRONMENT_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}environment${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${ENVIRONMENT_BRIDGE_CPP} ${ENVIRONMENT_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${ENVIRONMENT_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)

add_executable(rust_environment src/main.cpp  ${ENVIRONMENT_BRIDGE_CPP})

target_include_directories(
        rust_environment
        PRIVATE
        include/
        ${CARGO_TARGET_DIR}/cxxbridge/environment/src/
)

target_link_libraries(rust_environment ${ENVIRONMENT_LIB})
#target_link_libraries(rust_environment )

set_target_properties(
        rust_environment
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

