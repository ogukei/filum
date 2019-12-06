
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::vk::*;

use libc::{c_char, c_float, size_t, c_void};
use std::ffi::{CStr, CString};
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
    pub fn new(p_application_info: *const VkApplicationInfo) -> Self {
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

    pub fn with_command_buffer_wait(
        count: u32,
        buffers: *const VkCommandBuffer,
        wait_dst_stage_mask: *const VkPipelineStageFlags) -> Self {
        VkSubmitInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: wait_dst_stage_mask,
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

impl VkDescriptorPoolSize {
    pub fn new(descriptor_type: VkDescriptorType, count: u32) -> VkDescriptorPoolSize {
        VkDescriptorPoolSize {
            descriptorType: descriptor_type,
            descriptorCount: count,
        }
    }
}

impl VkDescriptorPoolCreateInfo {
    pub fn new(max_sets: u32, count: u32, sizes: *const VkDescriptorPoolSize) -> Self {
        VkDescriptorPoolCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            maxSets: max_sets,
            poolSizeCount: count,
            pPoolSizes: sizes,
        }
    }
}

impl VkDescriptorSetLayoutBinding {
    pub fn new(
        descriptor_type: VkDescriptorType, 
        stage_flags: u32,
        binding: u32) -> Self {
        VkDescriptorSetLayoutBinding {
            binding: binding,
            descriptorType: descriptor_type,
            descriptorCount: 1,
            stageFlags: stage_flags,
            pImmutableSamplers: ptr::null(),
        }
    }
}

impl VkDescriptorSetLayoutCreateInfo {
    pub fn new(count: u32, bindings: *const VkDescriptorSetLayoutBinding) -> Self {
        VkDescriptorSetLayoutCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            bindingCount: count,
            pBindings: bindings,
        }
    }
}

impl VkPipelineLayoutCreateInfo {
    pub fn new(count: u32, layouts: *const VkDescriptorSetLayout) -> Self {
        VkPipelineLayoutCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            setLayoutCount: count,
            pSetLayouts: layouts,
            pushConstantRangeCount: 0,
            pPushConstantRanges: ptr::null(),
        }
    }
}

impl VkDescriptorSetAllocateInfo {
    pub fn new(descriptor_pool: VkDescriptorPool, set_count: u32, layouts: *const VkDescriptorSetLayout) -> Self {
        VkDescriptorSetAllocateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: ptr::null(),
            descriptorPool: descriptor_pool,
            descriptorSetCount: set_count,
            pSetLayouts: layouts,
        }
    }
}

impl VkDescriptorBufferInfo {
    pub fn new(buffer: VkBuffer, offset: VkDeviceSize, range: VkDeviceSize) -> Self {
        VkDescriptorBufferInfo {
            buffer: buffer,
            offset: offset,
            range: range,
        }
    }
}

impl VkWriteDescriptorSet {
    pub fn new(
        dst_set: VkDescriptorSet, 
        descriptor_type: VkDescriptorType,
        dst_binding: u32,
        buffer_info: *const VkDescriptorBufferInfo,
    ) -> Self {
        VkWriteDescriptorSet {
            sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: dst_set,
            dstBinding: dst_binding,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: descriptor_type,
            pImageInfo: ptr::null(),
            pBufferInfo: buffer_info,
            pTexelBufferView: ptr::null(),
        }
    }
}

impl VkPipelineCacheCreateInfo {
    pub fn new() -> Self {
        VkPipelineCacheCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            initialDataSize: 0,
            pInitialData: ptr::null(),
        }
    }
}

impl VkPipelineShaderStageCreateInfo {
    pub fn new(
        stage: VkShaderStageFlagBits, 
        module: VkShaderModule,
        name: *const c_char,
        specialization_info: *const VkSpecializationInfo) -> Self {
        VkPipelineShaderStageCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            stage: stage,
            module: module,
            pName: name,
            pSpecializationInfo: specialization_info,
        }
    }
}

impl VkComputePipelineCreateInfo {
    pub fn new(
        stage: VkPipelineShaderStageCreateInfo,
        layout: VkPipelineLayout) -> Self {
        VkComputePipelineCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            stage: stage,
            layout: layout,
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0,
        }
    }
}

impl VkShaderModuleCreateInfo {
    pub fn new(code_size_bytes: size_t, code: *const u32) -> Self {
        VkShaderModuleCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            codeSize: code_size_bytes,
            pCode: code,
        }
    }
}

impl VkSpecializationMapEntry {
    pub fn new(constant_id: u32, offset: u32, size: size_t) -> Self {
        VkSpecializationMapEntry {
            constantID: constant_id,
            offset: offset,
            size: size,
        }
    }
}

impl VkSpecializationInfo {
    pub fn new(
        max_entry_count: u32, 
        entries: *const VkSpecializationMapEntry,
        data_size: size_t,
        data: *const c_void) -> Self {
        VkSpecializationInfo {
            mapEntryCount: max_entry_count,
            pMapEntries: entries,
            dataSize: data_size,
            pData: data,
        }
    }
}

impl VkBufferMemoryBarrier {
    pub fn new(
        src_access_mask: VkAccessFlags, 
        dst_access_mask: VkAccessFlags,
        buffer: VkBuffer,
        size: VkDeviceSize,
    ) -> Self {
        VkBufferMemoryBarrier {
            sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: src_access_mask,
            dstAccessMask: dst_access_mask,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            buffer: buffer,
            offset: 0,
            size: size,
        }
    }
}

impl VkPhysicalDeviceProperties {
    pub fn device_name(&self) -> CString {
        unsafe { CStr::from_ptr(self.deviceName.as_ptr()) }
            .to_owned()
    } 
}
