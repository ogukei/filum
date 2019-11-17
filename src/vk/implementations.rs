
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
            s_type: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: application_name,
            application_version: application_version,
            engine_name: engine_name,
            engine_version: engine_version,
            api_version: VK_API_VERSION_1_1,
        }
    }
}

impl VkInstanceCreateInfo {
    pub fn new(p_application_info: VkApplicationInfo) -> Self {
        VkInstanceCreateInfo { 
            s_type: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: 0,
            p_application_info: p_application_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
        }
    }
}
