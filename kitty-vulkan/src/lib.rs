#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::fmt;

pub use kitty_vulkan_sys as sys;

pub mod instance;

/// A wrapper around Vulkan's [`sys::VkResult`] to provide more descriptive error handling in Rust.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    derive_more::From,
    derive_more::Into,
    derive_more::Display,
    derive_more::Error,
)]
#[display("{:?}", _0)]
pub struct Error(#[error(not(source))] sys::VkResult);

/// A specialized `Result` type for Vulkan operations, using the custom `Error` type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Represents a Vulkan version with major, minor, and patch components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::From, derive_more::Into)]
pub struct Version(u32);

impl Version {
    /// Gets the Vulkan 1.0 version constant.
    pub const VULKAN_1_0: Self = Self::new_with_variant(0, 1, 0, 0);

    /// Gets the Vulkan 1.1 version constant.
    pub const VULKAN_1_1: Self = Self::new_with_variant(0, 1, 1, 0);

    /// Gets the Vulkan 1.2 version constant.
    pub const VULKAN_1_2: Self = Self::new_with_variant(0, 1, 2, 0);

    /// Gets the Vulkan 1.3 version constant.
    pub const VULKAN_1_3: Self = Self::new_with_variant(0, 1, 3, 0);

    /// Gets the Vulkan 1.4 version constant.
    pub const VULKAN_1_4: Self = Self::new_with_variant(0, 1, 4, 0);

    /// Creates a new `Version` instance with the specified major, minor, and patch numbers. The variant is set to 0 by default.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(sys::vk_api_make_version(0, major, minor, patch))
    }

    /// Creates a new `Version` instance with the specified variant, major, minor, and patch numbers.
    pub const fn new_with_variant(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self(sys::vk_api_make_version(variant, major, minor, patch))
    }

    /// Retrieves the variant component of the Vulkan version.
    pub const fn variant(&self) -> u32 {
        sys::vk_api_version_variant(self.0)
    }

    /// Retrieves the major version component of the Vulkan version.
    pub const fn major(&self) -> u32 {
        sys::vk_api_version_major(self.0)
    }

    /// Retrieves the minor version component of the Vulkan version.
    pub const fn minor(&self) -> u32 {
        sys::vk_api_version_minor(self.0)
    }

    /// Retrieves the patch version component of the Vulkan version.
    pub const fn patch(&self) -> u32 {
        sys::vk_api_version_patch(self.0)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.variant() == 0 {
            write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
        } else {
            write!(
                f,
                "{}.{}.{}.{}",
                self.variant(),
                self.major(),
                self.minor(),
                self.patch()
            )
        }
    }
}
