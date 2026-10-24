#![warn(clippy::pedantic)]

use cc::Build;
use std::{env, path::Path};

fn make_standard_build() -> Build {
    let mut build = Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings(false)
        .extra_warnings(false)
        .includes([
            "external/DirectX-Headers/include",
            "external/DirectXMath/Inc",
            "external/DirectXTex/DirectXTex",
        ]);

    if !cfg!(windows) {
        build
            .includes(["external/DirectX-Headers/include/wsl/stubs", "ffi/include"])
            .define("PAL_STDCPP_COMPAT", None);
    }

    let tool = build.get_compiler();
    if tool.is_like_gnu() {
        build.flag("-Wp,-w");
    }

    build
}

fn build_headers() {
    let root = Path::new("external/DirectX-Headers");
    let mut headers = make_standard_build();
    let mut guids = make_standard_build();
    let tool = headers.get_compiler();

    headers
        .include(root.join("include/directx"))
        .file(root.join("src/d3dx12_property_format_table.cpp"));
    guids
        .include(root.join("include/directx"))
        .file(root.join("src/dxguids.cpp"));

    if !cfg!(windows) && (tool.is_like_gnu() || tool.is_like_clang()) {
        headers.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
        guids.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
    }

    headers.compile("DirectX-Headers");
    guids.compile("DirectX-Guids");
}

fn build_tex() {
    let root = Path::new("external/DirectXTex");
    let mut build = make_standard_build();

    build.files(
        [
            "DirectXTex/BC.cpp",
            "DirectXTex/BC4BC5.cpp",
            "DirectXTex/BC6HBC7.cpp",
            "DirectXTex/DirectXTexCompress.cpp",
            "DirectXTex/DirectXTexConvert.cpp",
            "DirectXTex/DirectXTexDDS.cpp",
            "DirectXTex/DirectXTexHDR.cpp",
            "DirectXTex/DirectXTexImage.cpp",
            "DirectXTex/DirectXTexMipmaps.cpp",
            "DirectXTex/DirectXTexMisc.cpp",
            "DirectXTex/DirectXTexNormalMaps.cpp",
            "DirectXTex/DirectXTexPMAlpha.cpp",
            "DirectXTex/DirectXTexResize.cpp",
            "DirectXTex/DirectXTexTGA.cpp",
            "DirectXTex/DirectXTexUtil.cpp",
        ]
        .into_iter()
        .map(|x| root.join(x)),
    );

    if cfg!(windows) {
        build
            .files(
                [
                    "DirectXTex/DirectXTexFlipRotate.cpp",
                    "DirectXTex/DirectXTexWIC.cpp",
                ]
                .into_iter()
                .map(|x| root.join(x)),
            );
        println!("cargo:rustc-link-lib=ole32");

    }

    if cfg!(feature = "openmp") {
        if let Ok(flags) = env::var("DEP_OPENMP_FLAG"){
            flags
            .split(' ')
            .for_each(|f| {
                build.flag(f);
            });
        }
      
        if let Some(includes) = env::var_os("DEP_OPENMP_INCLUDE"){
            for f in env::split_paths(&includes).filter(|include| !include.as_os_str().is_empty()) {
                build.include(f);
            }
        }
    }

    build.compile("DirectXTex");

    if cfg!(feature = "openmp") {
        if let Some(link) = env::var_os("DEP_OPENMP_CARGO_LINK_INSTRUCTIONS") {
            for i in env::split_paths(&link).filter(|link| !link.as_os_str().is_empty()) {
                println!("cargo:{}", i.display());
            }
        }
    }
}

fn build_ffi() {
    let root = Path::new("ffi");
    let mut build = make_standard_build();
    build.file(root.join("main.cpp")).include(root);
    if cfg!(windows) {
        build.define("CONFIG_WINDOWS", "1");
    }
    build.compile("directxtex-ffi");
}

fn main() {
    build_headers();
    build_tex();
    build_ffi();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ffi/main.cpp");
}
