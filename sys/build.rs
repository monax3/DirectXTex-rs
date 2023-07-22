#![allow(clippy::expect_used)]

use std::env;
use std::path::Path;

use vcpkg::Library;

fn generate_bindings(library: &Library, bindings_file: &Path) {
    println!("cargo:print={:?}", library.include_paths);
    let bindings = bindgen::builder()
        .header("wrapper/DirectXTexWrapper.hpp")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .layout_tests(false)
        .disable_name_namespacing()
        .derive_default(true)
        .derive_debug(true)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global:   false,
        })
        .size_t_is_usize(true)
        .bitfield_enum("DirectX::.*_FLAGS")
        .no_debug("DirectX::TEX_DIMENSION")
        .newtype_enum("DirectX::TEX_ALPHA_MODE")
        .newtype_enum("DirectX::TEX_DIMENSION")
        .allowlist_function("DirectX::.*")
        .allowlist_function("Wrapper::.*")
        .allowlist_type("DirectX::.*")
        .allowlist_var("DirectX::.*")
        .allowlist_recursively(false)
        .blocklist_type("std::function")
        .blocklist_type("__m128")
        .blocklist_type(".*XMVECTOR.*")
        // inlined functions that are not all in the exported library
        .blocklist_function("DirectX::SaveToDDSMemory")
        .blocklist_function("DirectX::SaveToDDSFile")
        .blocklist_function("DirectX::GetMetadataFromTGAMemory")
        .blocklist_function("DirectX::GetMetadataFromTGAFile")
        .blocklist_function("DirectX::LoadFromTGAMemory")
        .blocklist_function("DirectX::LoadFromTGAFile")
        .blocklist_function("DirectX::SaveToTGAMemory")
        .blocklist_function("DirectX::SaveToTGAFile")
        .blocklist_function("DirectX::IsBGR")
        // functions using std::function that need a wrapper
        .blocklist_function("DirectX::GetMetadataFromWICMemory")
        .blocklist_function("DirectX::GetMetadataFromWICFile")
        .blocklist_function("DirectX::LoadFromWICMemory")
        .blocklist_function("DirectX::LoadFromWICFile")
        .blocklist_function("DirectX::SaveToWICMemory")
        .blocklist_function("DirectX::SaveToWICFile")
        .blocklist_function("DirectX::EvaluateImage")
        .blocklist_function("DirectX::TransformImage")
        .generate_cstr(true)
        // .allowlist_type("DXGI_FORMAT")
        .clang_args(["-x", "c++"])
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|inc| ["-I".to_string(), format!("{}", inc.display())])
                .flatten(),
        )
        .generate()
        .expect("Failed to generate bindings");

    // Replace void pointers in binding with u8 pointers to be more Rusty
    let fixed_bindings = bindings.to_string().replace("::core::ffi::c_void", "u8");

    std::fs::write(bindings_file, &fixed_bindings).expect("Failed to write bindings");
}

fn check_for_header(library: &Library, header: &str) -> bool {
    for path in &library.include_paths {
        let header_file = path.join(header);
        if header_file.is_file() {
            return true;
        }
    }
    false
}

fn main() {
    let bindings_file =
        Path::new(&env::var("OUT_DIR").expect("Failed to open OUT_DIR")).join("bindings.rs");

    let library = vcpkg::find_package("directxtex").expect("DirectXTex not found via vcpkg");
    generate_bindings(&library, &bindings_file);

    if !check_for_header(&library, "DirectXTex.h") {
        println!(
            "cargo:warning=DirectXTex.h not found in include paths, compilation will most likely \
             fail"
        );
    }
    if !check_for_header(&library, "DirectXTexEXR.h") {
        println!(
            "cargo:warning=DirectXTexEXR.h not found in include paths, ensure DirectXTex is \
             installed with EXR support"
        );
    }

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=ole32");

    let mut cc = cc::Build::new();

    if cc.get_compiler().is_like_clang() && std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        if std::env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |features| features.contains("crt-static"))
        {
            println!("cargo:warning=Setting static VC runtime");
            cc.flag("-fms-runtime-lib=static");
        } else {
            println!("cargo:warning=Setting dynamic VC runtime");
            cc.flag("-fms-runtime-lib=dll");
        }
    }

    cc.cpp(true)
        .includes(library.include_paths)
        .file("wrapper/DirectXTexWrapper.cpp")
        .warnings(true)
        .compile("DirectXTexWrapper");
}
