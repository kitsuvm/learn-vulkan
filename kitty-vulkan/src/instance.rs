//! Vulkan instance management.

use std::{ffi::CStr, ptr};

use bitflags::bitflags;

use crate::{
    Error, Result, Version,
    sys::{
        VkApplicationInfo, VkInstance_T, VkInstanceCreateInfo, VkResult, VkStructureType,
        vkCreateInstance, vkDestroyInstance,
    },
};

/// Application and engine information used when creating a Vulkan instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationInfo<'a> {
    /// The name of the application.
    pub app_name: &'a CStr,
    /// The version of the application.
    pub app_version: Version,
    /// The name of the engine used by the application.
    pub engine_name: &'a CStr,
    /// The version of the engine used by the application.
    pub engine_version: Version,
    /// The version of the Vulkan API that the application is designed to use.
    pub api_version: Version,
}

impl<'a> ApplicationInfo<'a> {
    /// Creates a new [`ApplicationInfo`] instance with the specified application and engine information, as well as the desired Vulkan API version.
    pub fn new(
        app_name: &'a CStr,
        app_version: Version,
        engine_name: &'a CStr,
        engine_version: Version,
        api_version: Version,
    ) -> Self {
        Self {
            app_name,
            app_version,
            engine_name,
            engine_version,
            api_version,
        }
    }

    /// Converts the [`ApplicationInfo`] instance into a `VkApplicationInfo` structure that can be used when creating a Vulkan instance.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because it returns a `VkApplicationInfo` structure that contains raw pointers to the application and engine names. The caller must ensure that these pointers remain valid for the duration of their use in Vulkan instance creation.
    pub unsafe fn to_vk_application_info(&self) -> VkApplicationInfo {
        VkApplicationInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pApplicationName: self.app_name.as_ptr(),
            applicationVersion: self.app_version.into(),
            pEngineName: self.engine_name.as_ptr(),
            engineVersion: self.engine_version.into(),
            apiVersion: self.api_version.into(),
            pNext: ptr::null(),
        }
    }
}

bitflags! {
    /// Flags used when creating a Vulkan instance, specifying additional options for instance creation.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InstanceCreateFlags: u32 {
        /// No flags specified.
        const NONE = 0;
        /// Indicates that the instance should be created with support for enumerating physical devices that are not directly supported by the Vulkan implementation, such as those provided by a compatibility layer. This flag is used in conjunction with the `VK_KHR_portability_enumeration` extension.
        const ENUMERATE_PORTABILITY = 0x00000001;
    }
}

/// A wrapper around a Vulkan instance, providing safe management of the underlying Vulkan instance resource.
pub struct Instance {
    /// A raw pointer to the Vulkan instance.
    instance: *mut VkInstance_T,
}

impl Instance {
    /// Creates a new Vulkan instance with the specified application and engine information, as well as the desired Vulkan API version. Returns a `Result` containing the created `Instance` or an `Error` if the instance creation fails.
    pub fn new(
        application_info: ApplicationInfo,
        layers: &[&CStr],
        extensions: &[&CStr],
        flags: InstanceCreateFlags,
    ) -> Result<Self> {
        let app_info = unsafe { application_info.to_vk_application_info() };

        let enabled_layers = if layers.is_empty() {
            ptr::null()
        } else {
            layers.as_ptr() as *const *const i8
        };

        let enabled_extensions = if extensions.is_empty() {
            ptr::null()
        } else {
            extensions.as_ptr() as *const *const i8
        };

        let create_info = VkInstanceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            flags: flags.bits(),
            pApplicationInfo: &app_info,
            enabledLayerCount: layers.len() as u32,
            ppEnabledLayerNames: enabled_layers,
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: enabled_extensions,
            pNext: ptr::null(),
        };

        let mut instance = ptr::null_mut();
        let result = unsafe { vkCreateInstance(&create_info, ptr::null(), &mut instance) };
        if result != VkResult::VK_SUCCESS {
            return Err(Error::from(result));
        };

        Ok(Self { instance })
    }

    /// Returns a raw pointer to the Vulkan instance.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because it returns a raw pointer to the Vulkan instance. The caller must ensure that the pointer is used safely and that the Vulkan instance is not destroyed while the pointer is still in use.
    pub unsafe fn instance(&self) -> *mut VkInstance_T {
        self.instance
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self.instance, ptr::null());
        }
    }
}
