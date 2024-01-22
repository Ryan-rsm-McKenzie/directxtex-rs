#![warn(clippy::pedantic)]

use cc::Build;
use std::path::Path;

fn build_headers() {
    let root = Path::new("external/DirectX-Headers");
    let mut headers = Build::new();
    let mut guids = Build::new();
    let tool = headers.get_compiler();

    headers
        .cpp(true)
        .std("c++14")
        .includes(
            ["include", "include/directx"]
                .into_iter()
                .map(|x| root.join(x)),
        )
        .file(root.join("src/d3dx12_property_format_table.cpp"));
    guids
        .cpp(true)
        .std("c++14")
        .includes(
            ["include", "include/directx"]
                .into_iter()
                .map(|x| root.join(x)),
        )
        .file(root.join("src/dxguids.cpp"));

    if !cfg!(windows) {
        headers.include(root.join("include/wsl/stubs"));
        guids.include(root.join("include/wsl/stubs"));
    } else if tool.is_like_gnu() || tool.is_like_clang() {
        headers.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
        guids.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
    }

    headers.compile("DirectX-Headers");
    guids.compile("DirectX-Guids");
}

fn build_tex() {
    let root = Path::new("external/DirectXTex");
    let mut build = Build::new();

    build
        .cpp(true)
        .std("c++17")
        .files(
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
        )
        .include(root.join("DirectXTex"))
        .includes([
            "external/DirectXMath/Inc",
            "external/DirectX-Headers/include",
        ]);

    if cfg!(windows) {
        build
            .files(
                [
                    "DirectXTex/DirectXTexFlipRotate.cpp",
                    "DirectXTex/DirectXTexWIC.cpp",
                ]
                .into_iter()
                .map(|x| root.join(x)),
            )
            .object("Ole32.lib");
    } else {
        build.include("external/DirectX-Headers/include/wsl/stubs");
    }

    build.compile("DirectXTex");
}

fn build_ffi() {
    let root = Path::new("ffi");
    let mut build = Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file(root.join("main.cpp"))
        .include(root)
        .include("external/DirectXTex/DirectXTex");
    build.compile("directxtex-ffi");
}

fn main() {
    build_headers();
    build_tex();
    build_ffi();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ffi/main.cpp");
}
