# DirectXTex Rust Bindings

These are unofficial Rust bindings for the DirectXTex texture processing library. A C++17 compliant compiler is required to build this library, and it is verified to work on Windows, Linux, and MacOS. The docs contain a migration table that describes the equivalency between the original C++ API and its Rust bindings. Please see [DirectXTex's README](https://github.com/microsoft/DirectXTex/blob/main/README.md) for more info.

The stable release docs are available at: <https://docs.rs/directxtex/latest/directxtex/>

Changelogs are available at: <https://github.com/Ryan-rsm-McKenzie/directxtex-rs/releases>


## OpenMP Support

DirectXTex uses [OpenMP](https://www.openmp.org/) for parallel processing. To enable OpenMP in this crate, build with the `openmp` feature:

### Requirements

Ensure that the OpenMP runtime (libomp) is installed on your system.

- On **Windows**, ensure the OpenMP runtime DLL `vcomp.dll` is available at runtime. You may bundle it with your application or rely on the MSVC redistributable to provide it.
  
- If your build system cannot locate OpenMP, specify the installation paths explicitly via environment variables `LIBRARY_PATH` and/or `CPATH`:
  ```sh
  export LIBRARY_PATH=/path/to/libomp/lib
  export CPATH=/path/to/libomp/include
  ```