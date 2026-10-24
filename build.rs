#![warn(clippy::pedantic)]

use std::{env, fs};
use cc::Build;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn compile_shaders() -> PathBuf {
    let root = Path::new("external/DirectXTex/DirectXTex");

    println!("cargo:rerun-if-env-changed=COMPILED_SHADERS");
    println!("cargo:rerun-if-env-changed=USE_PREBUILT_SHADERS");

    let use_prebuilt = env::var("USE_PREBUILT_SHADERS").is_ok_and(|value| {
        matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true")
    });

    if use_prebuilt {
        return env::var_os("COMPILED_SHADERS").map(PathBuf::from).expect(
            "Using prebuilt shaders requires the COMPILED_SHADERS variable is set",
        )
    }

    let shader_out = env::var_os("OUT_DIR").map(PathBuf::from).unwrap()
        .join("shaders");

    let fxc_installed = env::split_paths(&env::var_os("PATH").unwrap_or_default())
        .any(|dir| dir.join("fxc.exe").is_file());

    assert!(
        fxc_installed,
        "fxc.exe is required to compile DirectXTex shaders. \
         Install the Windows SDK containing FXC or \
         set USE_PREBUILT_SHADERS=1 and COMPILED_SHADERS \
         to a directory containing precompiled shader files."
    );

    fs::create_dir_all(&shader_out)
        .expect("Failed to create shader output directory");

    let shader_dir = root.join("Shaders");
    let script = shader_dir.join("CompileShaders.cmd");

    let status = Command::new("cmd")
        .args(["/C", "CompileShaders.cmd"])
        .current_dir(&shader_dir)
        .env("CompileShadersOutput", &shader_out)
        .status()
        .unwrap_or_else(|_| panic!("Failed to run {}", script.display()));

    assert!(
        status.success(),
        "DirectXTex shader compilation failed: {}",
        script.display()
    );

    println!("cargo:rerun-if-changed={}", script.display());

    for path in fs::read_dir(&shader_dir).unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
    {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    shader_out
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

        if cfg!(feature = "d3d11") {
            let shader_out = compile_shaders();
            build
                .include(shader_out)
                .files(
                [
                    "DirectXTex/BCDirectCompute.cpp",
                    "DirectXTex/DirectXTexCompressGPU.cpp",
                    "DirectXTex/DirectXTexD3D11.cpp"
                ].into_iter()
                .map(|x| root.join(x))
            );
        }
    }

    build.compile("DirectXTex");
}

fn build_ffi() {
    let root = Path::new("ffi");
    let mut build = make_standard_build();
    build.file(root.join("main.cpp")).include(root);
    if cfg!(all(windows, feature = "d3d11")) {
        build.file(root.join("d3d11.cpp"));
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
