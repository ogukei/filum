

use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;
use super::device::{Device, ShaderModule, CommandPool, BufferMemory};

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::marker::PhantomData;

pub struct CommandDispatch {
    compute_pipeline: Arc<ComputePipeline>,
    command_buffer: VkCommandBuffer,
    fence: VkFence,
}

impl CommandDispatch {
    pub fn new(compute_pipeline: &Arc<ComputePipeline>, workgroup_count: WorkgroupCount, push_constants: Vec<ConstantEntry>) -> Arc<Self> {
        let staging_buffer = compute_pipeline.staging_buffer();
        let command_pool = staging_buffer.command_pool();
        let device = command_pool.device();
        unsafe {
            let mut command_buffer = MaybeUninit::<VkCommandBuffer>::zeroed();
            {
                let alloc_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
                vkAllocateCommandBuffers(device.handle(), &alloc_info, command_buffer.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let command_buffer = command_buffer.assume_init();
            let mut fence = MaybeUninit::<VkFence>::zeroed();
            {
                let create_info = VkFenceCreateInfo::new(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as VkFlags);
                vkCreateFence(device.handle(), &create_info, ptr::null(), fence.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let fence = fence.assume_init();
            let begin_info = VkCommandBufferBeginInfo::new();
            vkBeginCommandBuffer(command_buffer, &begin_info)
                .into_result()
                .unwrap();
            // Barrier to ensure that input buffer transfer is finished before compute shader reads from it
            {
                let buffer_barrier = VkBufferMemoryBarrier::new(
                    VkAccessFlagBits::VK_ACCESS_HOST_WRITE_BIT as VkFlags,
                    VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkFlags,
                    staging_buffer.device_buffer_memory().buffer(),
                    0,
                    VK_WHOLE_SIZE,
                );
                vkCmdPipelineBarrier(
                    command_buffer,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_HOST_BIT as VkFlags,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as VkFlags,
                    VK_FLAGS_NONE,
                    0, ptr::null(),
                    1, &buffer_barrier,
                    0, ptr::null(),
                );
            }
            if !push_constants.is_empty() {
                let data = push_constants.iter()
                    .flat_map(|entry| entry.bytes.iter().cloned())
                    .collect::<Vec<u8>>();
                vkCmdPushConstants(
                    command_buffer,
                    compute_pipeline.layout,
                    VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT as u32,
                    0,
                    data.len() as u32,
                    data.as_ptr() as *const c_void,
                );
            }
            vkCmdBindPipeline(
                command_buffer,
                VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                compute_pipeline.handle);
            vkCmdBindDescriptorSets(
                command_buffer,
                VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                compute_pipeline.layout,
                0,
                1,
                &compute_pipeline.descriptor_set,
                0,
                ptr::null()
            );
            vkCmdDispatch(command_buffer, workgroup_count.x, workgroup_count.y, workgroup_count.z);
            // Barrier to ensure that shader writes are finished before buffer is read back from GPU
            {
                let buffer_barrier = VkBufferMemoryBarrier::new(
                    VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT as VkFlags,
                    VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as VkFlags |
                        VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkFlags,
                    staging_buffer.device_buffer_memory().buffer(),
                    0,
                    VK_WHOLE_SIZE,
                );
                vkCmdPipelineBarrier(
                    command_buffer,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as VkFlags,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkFlags |
                        VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as VkFlags,
                    VK_FLAGS_NONE,
                    0, ptr::null(),
                    1, &buffer_barrier,
                    0, ptr::null(),
                );
            }
            vkEndCommandBuffer(command_buffer);
            let command_dispatch = CommandDispatch {
                compute_pipeline: Arc::clone(compute_pipeline),
                command_buffer: command_buffer,
                fence: fence,
            };
            Arc::new(command_dispatch)
        }
    }

    pub fn dispatch(&self) {
        unsafe {
            let staging_buffer = self.compute_pipeline.staging_buffer();
            let device = staging_buffer.command_pool().device();
            let fence = self.fence;
            let command_buffer = self.command_buffer;
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let wait_mask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags;
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
        }
    }
}

impl Drop for CommandDispatch {
    fn drop(&mut self) {
        log_debug!("Drop CommandDispatch");
        unsafe {
            let staging_buffer = self.compute_pipeline.staging_buffer();
            let command_pool = staging_buffer.command_pool();
            let device = command_pool.device();
            vkDestroyFence(device.handle(), self.fence, ptr::null());
            self.fence = ptr::null_mut();
            vkFreeCommandBuffers(device.handle(), command_pool.handle(), 1, &self.command_buffer);
            self.command_buffer = ptr::null_mut();
        }
    }
}

pub struct WorkgroupCount {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct ConstantEntry {
    size: usize,
    bytes: Vec<u8>,
}

impl ConstantEntry {
    pub fn new(bytes: Vec<u8>) -> Self {
        ConstantEntry { size: bytes.len(), bytes }
    }
}

pub struct ComputePipeline {
    handle: VkPipeline,
    cache: VkPipelineCache,
    layout: VkPipelineLayout,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_layout: VkDescriptorSetLayout,
    descriptor_set: VkDescriptorSet,
    staging_buffer: Arc<StagingBuffer>,
    shader_module: Arc<ShaderModule>,
}

impl ComputePipeline {
    pub fn new(staging_buffer: &Arc<StagingBuffer>, 
        shader_module: &Arc<ShaderModule>, 
        spec_constants: Vec<ConstantEntry>) -> Arc<Self> {
        let command_pool = staging_buffer.command_pool();
        let device = command_pool.device();
        let regions = staging_buffer.regions();
        unsafe {
            let mut descriptor_pool = MaybeUninit::<VkDescriptorPool>::zeroed();
            {
                let size = VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1);
                let create_info = VkDescriptorPoolCreateInfo::new(1, 1, &size);
                vkCreateDescriptorPool(device.handle(), &create_info, ptr::null(), descriptor_pool.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let descriptor_pool = descriptor_pool.assume_init();
            let mut descriptor_set_layout = MaybeUninit::<VkDescriptorSetLayout>::zeroed();
            {
                let bindings = regions.iter()
                    .enumerate()
                    .map(|(index, _)| {
                        VkDescriptorSetLayoutBinding::new(
                            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                            VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT as u32,
                            index as u32,
                        )
                    })
                    .collect::<Vec<VkDescriptorSetLayoutBinding>>();
                let create_info = VkDescriptorSetLayoutCreateInfo::new(bindings.len() as u32, bindings.as_ptr());
                vkCreateDescriptorSetLayout(device.handle(), &create_info, ptr::null(), descriptor_set_layout.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let descriptor_set_layout = descriptor_set_layout.assume_init();
            let mut pipeline_layout = MaybeUninit::<VkPipelineLayout>::zeroed();
            {
                let create_info = VkPipelineLayoutCreateInfo::new(1, &descriptor_set_layout);
                vkCreatePipelineLayout(device.handle(), &create_info, ptr::null(), pipeline_layout.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let pipeline_layout = pipeline_layout.assume_init();
            let mut descriptor_set = MaybeUninit::<VkDescriptorSet>::zeroed();
            {
                let alloc_info = VkDescriptorSetAllocateInfo::new(descriptor_pool, 1, &descriptor_set_layout);
                vkAllocateDescriptorSets(device.handle(), &alloc_info, descriptor_set.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let descriptor_set = descriptor_set.assume_init();
            {
                let infos = regions.iter()
                    .map(|region| {
                        VkDescriptorBufferInfo::new(
                            staging_buffer.device_buffer_memory().buffer(), 
                            region.offset(), 
                            region.size())
                    })
                    .collect::<Vec<VkDescriptorBufferInfo>>();
                let write_sets = infos.iter()
                    .enumerate()
                    .map(|(index, buffer_info)| {
                        VkWriteDescriptorSet::new(
                            descriptor_set, 
                            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                            index as u32, 
                            buffer_info)
                    })
                    .collect::<Vec<VkWriteDescriptorSet>>();
                vkUpdateDescriptorSets(device.handle(), write_sets.len() as u32, write_sets.as_ptr(), 0, ptr::null());
            }
            // Pipeline Cache
            let mut pipeline_cache = MaybeUninit::<VkPipelineCache>::zeroed();
            {
                let create_info = VkPipelineCacheCreateInfo::new();
                vkCreatePipelineCache(device.handle(), &create_info, ptr::null(), pipeline_cache.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let pipeline_cache = pipeline_cache.assume_init();
            let mut compute_pipeline = MaybeUninit::<VkPipeline>::zeroed();
            {
                let data = spec_constants.iter()
                    .flat_map(|entry| entry.bytes.iter().cloned())
                    .collect::<Vec<u8>>();
                let entries = spec_constants.iter()
                    .enumerate()
                    .scan(0usize, |state, (index, entry)| {
                        let offset = *state;
                        let entry = VkSpecializationMapEntry::new(
                            index as u32, 
                            offset as u32,
                            entry.size);
                        *state += entry.size;
                        Some(entry)
                    })
                    .collect::<Vec<VkSpecializationMapEntry>>();
                let spec_info = VkSpecializationInfo::new(
                    entries.len() as u32,
                    entries.as_ptr(),
                    data.len(),
                    data.as_ptr() as *const c_void
                );
                let name = CString::new("main").unwrap();
                let stage = VkPipelineShaderStageCreateInfo::new(
                    VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT,
                    shader_module.handle(),
                    name.as_ptr(),
                    &spec_info
                );
                let create_info = VkComputePipelineCreateInfo::new(stage, pipeline_layout);
                vkCreateComputePipelines(device.handle(), pipeline_cache, 1, &create_info, ptr::null(), compute_pipeline.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let compute_pipeline = compute_pipeline.assume_init();
            let compute_pipeline = ComputePipeline {
                handle: compute_pipeline,
                cache: pipeline_cache,
                layout: pipeline_layout,
                descriptor_pool: descriptor_pool,
                descriptor_set_layout: descriptor_set_layout,
                descriptor_set: descriptor_set,
                shader_module: Arc::clone(shader_module),
                staging_buffer: Arc::clone(staging_buffer),
            };
            Arc::new(compute_pipeline)
        }
    }

    pub fn staging_buffer(&self) -> &Arc<StagingBuffer> {
        &self.staging_buffer
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        log_debug!("Drop ComputePipeline");
        unsafe {
            let command_pool = self.staging_buffer.command_pool();
            let device = command_pool.device();
            vkDestroyPipelineLayout(device.handle(), self.layout, ptr::null());
            self.layout = ptr::null_mut();
            vkDestroyDescriptorSetLayout(device.handle(), self.descriptor_set_layout, ptr::null());
            self.descriptor_set_layout = ptr::null_mut();
            vkDestroyDescriptorPool(device.handle(), self.descriptor_pool, ptr::null());
            self.descriptor_pool = ptr::null_mut();
            vkDestroyPipeline(device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
            vkDestroyPipelineCache(device.handle(), self.cache, ptr::null());
            self.cache = ptr::null_mut();
        }
    }
}

pub struct StagingBuffer {
    buffer_size: VkDeviceSize,
    regions: Vec<StagingBufferRegion>,
    host_buffer_memory: Arc<BufferMemory>,
    device_buffer_memory: Arc<BufferMemory>,
    command_pool: Arc<CommandPool>,
    mapped: *mut c_void,
}

impl StagingBuffer {
    pub fn new(command_pool: &Arc<CommandPool>, region_sizes: &[usize]) -> Arc<Self> {
        let device = command_pool.device();
        // TODO:
        // VkPhysicalDeviceLimits::nonCoherentAtomSize
        let buffer_size = region_sizes.iter()
            .map(|v| *v as VkDeviceSize)
            .sum();
        // host buffer
        let host_buffer_memory = BufferMemory::new(
            device,
            VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32, 
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_CACHED_BIT as u32,
            buffer_size).unwrap();
        // device buffer
        let device_buffer_memory = BufferMemory::new(
            device,
            VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32 |
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            buffer_size).unwrap();
        // mapping
        let mapped: *mut c_void;
        unsafe {
            let mut maybe_mapped = MaybeUninit::<*mut c_void>::zeroed();
            vkMapMemory(device.handle(), host_buffer_memory.memory(), 0, buffer_size, 0, maybe_mapped.as_mut_ptr())
                .into_result()
                .unwrap();
            mapped = maybe_mapped.assume_init();
        }
        // regions
        let regions = region_sizes.iter()
            .map(|v| *v as VkDeviceSize)
            .scan(0 as VkDeviceSize, |state, size| {
                let offset = *state;
                let region = StagingBufferRegion::new(offset, size, 
                    command_pool, 
                    &host_buffer_memory, 
                    &device_buffer_memory,
                    mapped);
                *state += size;
                Some(region)
            })
            .collect::<Vec<_>>();
        let staging_buffer = StagingBuffer {
            buffer_size: buffer_size,
            regions: regions,
            host_buffer_memory: host_buffer_memory,
            device_buffer_memory: device_buffer_memory,
            command_pool: Arc::clone(command_pool),
            mapped: mapped,
        };
        Arc::new(staging_buffer)
    }

    pub fn write_region_with_slice<ItemType>(&self, region_index: usize, access: impl FnOnce(&mut [ItemType])) {
        let region = self.nth_region(region_index)
        .unwrap();
        unsafe {
            access(region.as_mut_slice::<ItemType>());
        }
        region.flush_mapped_memory_range();
        region.transfer_host_to_device();
    }

    pub fn write_region<DataType>(&self, region_index: usize, access: impl FnOnce(&mut DataType)) {
        let region = self.nth_region(region_index)
            .unwrap();
        unsafe {
            access(region.as_mut());
        }
        region.flush_mapped_memory_range();
        region.transfer_host_to_device();
    }

    pub fn read_region_with_slice<ItemType>(&self, region_index: usize, access: impl FnOnce(&[ItemType])) {
        let region = self.nth_region(region_index)
            .unwrap();
        region.transfer_device_to_host();
        region.invalidate_mapped_memory_range();
        unsafe {
            access(region.as_slice::<ItemType>());
        }
    }

    pub fn read_region<DataType>(&self, region_index: usize, access: impl FnOnce(&DataType)) {
        let region = self.nth_region(region_index)
            .unwrap();
        region.transfer_device_to_host();
        region.invalidate_mapped_memory_range();
        unsafe {
            access(region.as_ref::<DataType>());
        }
    }

    #[inline]
    pub fn command_pool(&self) -> &Arc<CommandPool> {
        &self.command_pool
    }

    #[inline]
    pub fn host_buffer_memory(&self) -> &Arc<BufferMemory> {
        &self.host_buffer_memory
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &Arc<BufferMemory> {
        &self.device_buffer_memory
    }

    #[inline]
    fn nth_region(&self, index: usize) -> Option<&StagingBufferRegion> {
        self.regions.get(index)
    }

    #[inline]
    fn regions(&self) -> &Vec<StagingBufferRegion> {
        &self.regions
    }
}

impl Drop for StagingBuffer {
    fn drop(&mut self) {
        log_debug!("Drop StagingBuffer");
        let device = self.command_pool.device();
        let host_buffer_memory = self.host_buffer_memory();
        unsafe {
            vkUnmapMemory(device.handle(), host_buffer_memory.memory());
        }
    }
}

pub struct StagingBufferRegion {
    copy_region: VkBufferCopy,
    command_pool: Arc<CommandPool>,
    host_buffer_memory: Arc<BufferMemory>,
    device_buffer_memory: Arc<BufferMemory>,
    host_to_device_command: VkCommandBuffer,
    device_to_host_command: VkCommandBuffer,
    host_to_device_fence: VkFence,
    device_to_host_fence: VkFence,
    region_ptr: *mut u8,
}

impl StagingBufferRegion {
    // depends on staging buffer as long as its host buffer is mapped
    // so that new() returns StagingBufferRegion instead of Arc<StagingBufferRegion>
    pub fn new(
        offset: VkDeviceSize, 
        size: VkDeviceSize,
        command_pool: &Arc<CommandPool>,
        host_buffer_memory: &Arc<BufferMemory>,
        device_buffer_memory: &Arc<BufferMemory>,
        mapped: *mut c_void) -> StagingBufferRegion {
        let copy_region = VkBufferCopy::new(offset, size);
        let device = command_pool.device();
        unsafe {
            let mut host_to_device_command = MaybeUninit::<VkCommandBuffer>::zeroed();
            let mut device_to_host_command = MaybeUninit::<VkCommandBuffer>::zeroed();
            let mut host_to_device_fence = MaybeUninit::<VkFence>::zeroed();
            let mut device_to_host_fence = MaybeUninit::<VkFence>::zeroed();
            {
                let alloc_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
                vkAllocateCommandBuffers(device.handle(), &alloc_info, host_to_device_command.as_mut_ptr())
                    .into_result()
                    .unwrap();
                let create_info = VkFenceCreateInfo::new(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as VkFlags);
                vkCreateFence(device.handle(), &create_info, ptr::null(), host_to_device_fence.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            {
                let alloc_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
                vkAllocateCommandBuffers(device.handle(), &alloc_info, device_to_host_command.as_mut_ptr())
                    .into_result()
                    .unwrap();
                let create_info = VkFenceCreateInfo::new(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as VkFlags);
                vkCreateFence(device.handle(), &create_info, ptr::null(), device_to_host_fence.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let host_to_device_command = host_to_device_command.assume_init();
            {
                let begin_info = VkCommandBufferBeginInfo::new();
                vkBeginCommandBuffer(host_to_device_command, &begin_info)
                    .into_result()
                    .unwrap();
                // copy to staging buffer
                vkCmdCopyBuffer(
                    host_to_device_command,
                    host_buffer_memory.buffer(), 
                    device_buffer_memory.buffer(), 
                    1,
                    &copy_region
                );
                vkEndCommandBuffer(host_to_device_command);
            }
            let device_to_host_command = device_to_host_command.assume_init();
            {
                let begin_info = VkCommandBufferBeginInfo::new();
                vkBeginCommandBuffer(device_to_host_command, &begin_info)
                    .into_result()
                    .unwrap();
                // Submission guarantees the host write being complete, as per
                // https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html#synchronization-submission-host-writes
                // So no need for a barrier before the transfer for that purpose.
                // Read back to host visible buffer
                vkCmdCopyBuffer(
                    device_to_host_command, 
                    device_buffer_memory.buffer(),
                    host_buffer_memory.buffer(),
                    1,
                    &copy_region);
                // Barrier to ensure that buffer copy is finished before host reading from it
                {
                    let buffer_barrier = VkBufferMemoryBarrier::new(
                        VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as VkFlags,
                        VkAccessFlagBits::VK_ACCESS_HOST_READ_BIT as VkFlags,
                        host_buffer_memory.buffer(),
                        offset,
                        size,
                    );
                    vkCmdPipelineBarrier(
                        device_to_host_command,
                        VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkFlags,
                        VkPipelineStageFlagBits::VK_PIPELINE_STAGE_HOST_BIT as VkFlags,
                        VK_FLAGS_NONE,
                        0, ptr::null(),
                        1, &buffer_barrier,
                        0, ptr::null(),
                    );
                }
                vkEndCommandBuffer(device_to_host_command);
            }
            let region = StagingBufferRegion {
                copy_region: copy_region,
                command_pool: Arc::clone(command_pool),
                host_buffer_memory: Arc::clone(host_buffer_memory),
                device_buffer_memory: Arc::clone(device_buffer_memory),
                host_to_device_command,
                device_to_host_command,
                host_to_device_fence: host_to_device_fence.assume_init(),
                device_to_host_fence: device_to_host_fence.assume_init(),
                region_ptr: (mapped as *mut u8).offset(offset as isize),
            };
            region
        }
    }

    pub fn transfer_host_to_device(&self) {
        let device = self.command_pool.device();
        let command_buffer = self.host_to_device_command;
        let fence = self.host_to_device_fence;
        unsafe {
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let wait_mask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags;
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
        }
    }

    pub fn transfer_device_to_host(&self) {
        let device = self.command_pool.device();
        let command_buffer = self.device_to_host_command;
        let fence = self.device_to_host_fence;
        unsafe {
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let wait_mask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags;
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
        }
    }

    fn invalidate_mapped_memory_range(&self) {
        let device = self.host_buffer_memory.device();
        let memory = self.host_buffer_memory.memory();
        let mapped_range = VkMappedMemoryRange::new(memory, self.offset(), self.size());
        unsafe {
            vkInvalidateMappedMemoryRanges(device.handle(), 1, &mapped_range)
                .into_result()
                .unwrap();
        }
    }

    fn flush_mapped_memory_range(&self) {
        let device = self.host_buffer_memory.device();
        let memory = self.host_buffer_memory.memory();
        let mapped_range = VkMappedMemoryRange::new(memory, self.offset(), self.size());
        unsafe {
            vkFlushMappedMemoryRanges(device.handle(), 1, &mapped_range)
                .into_result()
                .unwrap();
        }
    }

    #[inline]
    pub fn offset(&self) -> VkDeviceSize {
        self.copy_region.srcOffset
    }

    #[inline]
    pub fn size(&self) -> VkDeviceSize {
        self.copy_region.size
    }

    #[inline]
    pub fn copy_region(&self) -> VkBufferCopy {
        self.copy_region
    }

    #[inline]
    unsafe fn as_slice<'a, ValueType>(&'a self) -> &'a [ValueType] {
        let value_size = std::mem::size_of::<ValueType>();
        let length = self.size() as usize / value_size;
        std::slice::from_raw_parts(self.region_ptr as *const ValueType, length)
    }

    #[inline]
    unsafe fn as_ref<'a, ValueType>(&'a self) -> &'a ValueType {
        (self.region_ptr as *const ValueType).as_ref().unwrap()
    }

    #[inline]
    unsafe fn as_mut_slice<'a, ValueType>(&'a self) -> &'a mut [ValueType] {
        let value_size = std::mem::size_of::<ValueType>();
        let length = self.size() as usize / value_size;
        std::slice::from_raw_parts_mut(self.region_ptr as *mut ValueType, length)
    }

    #[inline]
    unsafe fn as_mut<'a, ValueType>(&'a self) -> &'a mut ValueType {
        (self.region_ptr as *mut ValueType).as_mut().unwrap()
    }
}

impl Drop for StagingBufferRegion {
    fn drop(&mut self) {
        log_debug!("Drop StagingBufferRegion");
        unsafe {
            let command_pool = &self.command_pool;
            let device = command_pool.device();
            vkDestroyFence(device.handle(), self.host_to_device_fence, ptr::null());
            self.host_to_device_fence = ptr::null_mut();
            vkDestroyFence(device.handle(), self.device_to_host_fence, ptr::null());
            self.device_to_host_fence = ptr::null_mut();
            vkFreeCommandBuffers(device.handle(), command_pool.handle(), 1, &self.host_to_device_command);
            self.host_to_device_command = ptr::null_mut();
            vkFreeCommandBuffers(device.handle(), command_pool.handle(), 1, &self.device_to_host_command);
            self.device_to_host_command = ptr::null_mut();
        }
    }
}