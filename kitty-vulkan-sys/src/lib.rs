#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Constructs a Vulkan API version number from its components.
pub const fn vk_api_make_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

/// Extracts the variant component from a Vulkan API version number.
pub const fn vk_api_version_variant(version: u32) -> u32 {
    version >> 29
}

/// Extracts the major version component from a Vulkan API version number.
pub const fn vk_api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7F
}

/// Extracts the minor version component from a Vulkan API version number.
pub const fn vk_api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}

/// Extracts the patch version component from a Vulkan API version number.
pub const fn vk_api_version_patch(version: u32) -> u32 {
    version & 0xFFF
}

mod bindings;

pub use bindings::*;
