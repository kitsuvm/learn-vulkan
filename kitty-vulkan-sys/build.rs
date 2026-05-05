//! Build script for the [`kitty-vulkan-sys`] crate.

use std::{env, path::PathBuf};

/// The entry point of the build script.
fn main() {
    if let Some(libclang_path) = find_libclang() {
        unsafe {
            // Set the LIBCLANG_PATH environment variable for bindgen to find the libclang library.
            env::set_var("LIBCLANG_PATH", libclang_path);
        }
    }

    let vulkan_sdk_path = find_vulkan_sdk();

    // If a Vulkan SDK path is found and we're on Windows, add the SDK's Lib directory to the linker search path.
    if let Some(ref path) = vulkan_sdk_path
        && cfg!(target_os = "windows")
    {
        println!("cargo:rustc-link-search=native={}/Lib", path);
    }

    // Link against the Vulkan library, using static linking if the "crt-static" feature is enabled, otherwise using dynamic linking.
    let link_kind = if cfg!(target_feature = "crt-static") {
        "static"
    } else {
        "dylib"
    };

    // Link against the Vulkan library with the appropriate link kind.
    println!("cargo::rustc-link-lib={}=vulkan-1", link_kind);

    // Generate Rust bindings for the Vulkan API using bindgen, configuring it to include the appropriate header.
    let mut vulkan_bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // If a Vulkan SDK path is found and we're on Windows, add the SDK's Include directory to the bindgen builder's clang arguments.
    if let Some(ref path) = vulkan_sdk_path
        && cfg!(target_os = "windows")
    {
        vulkan_bindings_builder = vulkan_bindings_builder.clang_arg(format!("-I{}/Include", path));
    }

    // If we're on Windows, define the VK_USE_PLATFORM_WIN32_KHR macro for bindgen to ensure platform-specific Vulkan definitions are included.
    if cfg!(target_os = "windows") {
        vulkan_bindings_builder = vulkan_bindings_builder.clang_arg("-DVK_USE_PLATFORM_WIN32_KHR");
    }

    // Configure bindgen to rustify some enums, then generate the bindings and write them to the output directory.
    let vulkan_bindings = vulkan_bindings_builder
        .rustified_enum("VkResult")
        .rustified_enum("VkStructureType")
        .generate()
        .expect("Unable to generate bindings for Vulkan API");

    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    vulkan_bindings
        .write_to_file(output_path.join("bindings.rs"))
        .expect("Couldn't write bindings for Vulkan API!");
}

/// Attempts to find the libclang library by checking the LIBCLANG_PATH environment variable, returning the path if found.
fn find_libclang() -> Option<String> {
    // We don't need to search for libclang using the environment variable as we use this function to set it for bindgen.
    // if let Ok(path) = env::var("LIBCLANG_PATH") {
    //     return Some(path);
    // }

    cfg_select! {
        windows => search_libclang(),
        _ => None
    }
}

#[cfg(windows)]
/// Searches common installation directories for LLVM to find the libclang library, returning the path if found.
fn search_libclang() -> Option<String> {
    use windows::Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{
            FOLDERID_ProgramFiles, FOLDERID_ProgramFilesX86, KF_FLAG_DEFAULT, SHGetKnownFolderPath,
        },
    };

    let programs_files = unsafe {
        SHGetKnownFolderPath(&FOLDERID_ProgramFiles, KF_FLAG_DEFAULT, None)
            .ok()
            .and_then(|path| {
                let path_str = path.to_string().ok();
                CoTaskMemFree(Some(path.as_ptr() as _));
                path_str
            })
    };

    if let Some(programs_files) = programs_files {
        let libclang_dir = PathBuf::from(programs_files).join("LLVM").join("bin");
        if libclang_dir.is_dir() {
            return libclang_dir.to_str().map(ToOwned::to_owned);
        } else {
            let programs_files_x86 = unsafe {
                SHGetKnownFolderPath(&FOLDERID_ProgramFilesX86, KF_FLAG_DEFAULT, None)
                    .ok()
                    .and_then(|path| {
                        let path_str = path.to_string().ok();
                        CoTaskMemFree(Some(path.as_ptr() as _));
                        path_str
                    })
            };

            if let Some(programs_files_x86) = programs_files_x86 {
                let libclang_dir = PathBuf::from(programs_files_x86).join("LLVM").join("bin");
                if libclang_dir.is_dir() {
                    return libclang_dir.to_str().map(ToOwned::to_owned);
                }
            }
        }
    }

    None
}

/// Attempts to find the Vulkan SDK path by checking the VULKAN_SDK environment variable or searching common installation directories.
fn find_vulkan_sdk() -> Option<String> {
    if let Ok(path) = env::var("VULKAN_SDK") {
        return Some(path);
    }

    cfg_select! {
        windows => search_vulkan_sdk(),
        _ => None
    }
}

#[cfg(windows)]
/// Searches the C:/VulkanSDK directory for installed Vulkan SDK versions and returns the path of the latest version found.
fn search_vulkan_sdk() -> Option<String> {
    use std::fs;

    let vulkan_dir = PathBuf::from("C:/VulkanSDK");

    let mut versions = Vec::new();
    if vulkan_dir.is_dir() {
        let entries = fs::read_dir(vulkan_dir).ok()?;

        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir()
                && let Some(entry_path_str) = entry_path.to_str()
                && let Some(version) = entry_path
                    .file_name()
                    .and_then(|v| v.to_str())
                    .and_then(|v| semver::Version::parse(v).ok())
            {
                versions.push((version, entry_path_str.to_string()));
            }
        }
    }

    versions
        .into_iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, version)| version)
}
