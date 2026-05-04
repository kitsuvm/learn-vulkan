use crate::{
    VkResult, VkResult_VK_ERROR_INCOMPATIBLE_DRIVER, VkResult_VK_SUCCESS,
};


mod bindings;
pub use bindings::*;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::From)]
pub struct Error(VkResult);

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
