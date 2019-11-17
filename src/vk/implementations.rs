
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::vk::*;

use libc::{c_char};
use std::ptr;

const VK_API_VERSION_1_1: u32 = 4198400;

impl VkApplicationInfo {
    pub fn new(
        application_name: *const c_char,
        application_version: u32,
        engine_name: *const c_char,
        engine_version: u32) -> Self {
        
        VkApplicationInfo { 
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: application_name,
            applicationVersion: application_version,
            pEngineName: engine_name,
            engineVersion: engine_version,
            apiVersion: VK_API_VERSION_1_1,
        }
    }
}

impl VkInstanceCreateInfo {
    pub fn new(p_application_info: VkApplicationInfo) -> Self {
        VkInstanceCreateInfo { 
            sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            pApplicationInfo: p_application_info,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
        }
    }
}

impl VkQueueFamilyProperties {
    pub fn new() -> Self {
        VkQueueFamilyProperties {
            queueFlags: 0,
            queueCount: 0,
            timestampValidBits: 0,
            minImageTransferGranularity: VkExtent3D::new()
        }
    }
}

impl VkExtent3D {
    pub fn new() -> Self {
        VkExtent3D { width: 0, height: 0, depth: 0 }
    }
}