
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::vk::*;

use libc::{c_char, c_float};
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

    pub fn has_compute_queue_bit(&self) -> bool {
        (self.queueFlags & (VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32)) != 0
    }
}

impl VkExtent3D {
    pub fn new() -> Self {
        VkExtent3D { width: 0, height: 0, depth: 0 }
    }
}

impl VkDeviceQueueCreateInfo {
    pub fn new(family_index: u32, queue_count: u32, queue_priorities: *const c_float) -> Self {
        VkDeviceQueueCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueFamilyIndex: family_index,
            queueCount: queue_count,
            pQueuePriorities: queue_priorities,
        }
    }
}

impl VkDeviceCreateInfo {
    pub fn new(
        create_queue_info_count: u32, 
        create_queue_infos: *const VkDeviceQueueCreateInfo) -> Self {
        VkDeviceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            queueCreateInfoCount: create_queue_info_count,
            pQueueCreateInfos: create_queue_infos,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
            pEnabledFeatures: ptr::null(),
        }
    }
}