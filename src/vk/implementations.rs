
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
    pub fn new(
        family_index: u32, 
        queue_count: u32, 
        queue_priorities: *const c_float) -> Self {

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

impl VkCommandPoolCreateInfo {
    pub fn new(queue_family_index: u32) -> Self {
        VkCommandPoolCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,
            queueFamilyIndex: queue_family_index,
        }
    }
}

impl VkBufferCreateInfo {
    pub fn new(
        size: VkDeviceSize,
        usage_flags: VkBufferUsageFlags,
        sharing_mode: VkSharingMode,
    ) -> Self {
        VkBufferCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            size: size,
            usage: usage_flags,
            sharingMode: sharing_mode,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null(),
        }
    }
}

impl VkMemoryAllocateInfo {
    pub fn new(allocation_size: VkDeviceSize, memory_type_index: u32) -> Self {
        VkMemoryAllocateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            pNext: ptr::null(),
            allocationSize: allocation_size,
            memoryTypeIndex: memory_type_index,
        }
    }
}

impl VkMappedMemoryRange {
    pub fn new(memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        VkMappedMemoryRange {
            sType: VkStructureType::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: ptr::null(),
            memory: memory,
            offset: offset,
            size: size,
        }
    }
}

impl VkCommandBufferAllocateInfo {
    pub fn new(command_pool: VkCommandPool, level: VkCommandBufferLevel, command_buffer_count: u32) -> Self {
        VkCommandBufferAllocateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: command_pool,
            level: level,
            commandBufferCount: command_buffer_count,
        }
    }
}

impl VkCommandBufferBeginInfo {
    pub fn new() -> Self {
        VkCommandBufferBeginInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            flags: 0,
            pInheritanceInfo: ptr::null(),
        }
    }
}

impl VkBufferCopy {
    pub fn new(size: VkDeviceSize) -> Self {
        VkBufferCopy {
            srcOffset: 0,
            dstOffset: 0,
            size: size,
        }
    }
}

impl VkSubmitInfo {
    pub fn with_command_buffer(count: u32, buffers: *const VkCommandBuffer) -> Self {
        VkSubmitInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: count,
            pCommandBuffers: buffers,
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null(),
        }
    }
}

impl VkFenceCreateInfo {
    pub fn new(flags: VkFenceCreateFlags) -> Self {
        VkFenceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags,
        }
    }
}