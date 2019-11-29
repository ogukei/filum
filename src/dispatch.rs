

use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;
use super::device::{Device, ShaderModule, CommandPool, BufferMemory};

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};

use std::io::Read;

pub struct CommandDispatch {
    pub output: Vec<u32>,
}

impl CommandDispatch {
    pub fn new(pipeline: &ComputePipeline) -> Self {
        let staging_buffer = pipeline.staging_buffer();
        let device = staging_buffer.command_pool().device();
        unsafe {
            let command_buffer = pipeline.command_buffer;
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
            vkCmdBindPipeline(
                command_buffer,
                VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline.handle);
            vkCmdBindDescriptorSets(
                command_buffer,
                VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline.layout,
                0,
                1,
                &pipeline.descriptor_set,
                0,
                ptr::null()
            );
            vkCmdDispatch(command_buffer, staging_buffer.buffer_element_count as u32, 1, 1);
            // Barrier to ensure that shader writes are finished before buffer is read back from GPU
            {
                let buffer_barrier = VkBufferMemoryBarrier::new(
                    VkAccessFlagBits::VK_ACCESS_SHADER_WRITE_BIT as VkFlags,
                    VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as VkFlags,
                    staging_buffer.device_buffer_memory().buffer(),
                    VK_WHOLE_SIZE,
                );
                vkCmdPipelineBarrier(
                    command_buffer,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT as VkFlags,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkFlags,
                    VK_FLAGS_NONE,
                    0, ptr::null(),
                    1, &buffer_barrier,
                    0, ptr::null(),
                );
            }
            // Read back to host visible buffer
            let copy_region = VkBufferCopy::new(staging_buffer.buffer_size);
            vkCmdCopyBuffer(
                command_buffer, 
                staging_buffer.device_buffer_memory().buffer(),
                staging_buffer.host_buffer_memory().buffer(),
                1,
                &copy_region);
            // Barrier to ensure that buffer copy is finished before host reading from it
            {
                let buffer_barrier = VkBufferMemoryBarrier::new(
                    VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as VkFlags,
                    VkAccessFlagBits::VK_ACCESS_HOST_READ_BIT as VkFlags,
                    staging_buffer.host_buffer_memory().buffer(),
                    VK_WHOLE_SIZE,
                );
                vkCmdPipelineBarrier(
                    command_buffer,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkFlags,
                    VkPipelineStageFlagBits::VK_PIPELINE_STAGE_HOST_BIT as VkFlags,
                    VK_FLAGS_NONE,
                    0, ptr::null(),
                    1, &buffer_barrier,
                    0, ptr::null(),
                );
            }
            vkEndCommandBuffer(command_buffer);
            // submit compute work
            let fence = pipeline.fence;
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let wait_mask = VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags;
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
            // Make device writes visible to the host
            let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
            vkMapMemory(device.handle(), staging_buffer.host_buffer_memory().memory(), 0, VK_WHOLE_SIZE, 0, mapped.as_mut_ptr());
            let mapped = mapped.assume_init();
            let mapped_range = VkMappedMemoryRange::new(staging_buffer.host_buffer_memory().memory(), 0, VK_WHOLE_SIZE);
            vkInvalidateMappedMemoryRanges(device.handle(), 1, &mapped_range);
            let mut output: Vec<u32> = Vec::with_capacity(staging_buffer.buffer_element_count);
            {
                output.resize(staging_buffer.buffer_element_count, 0);
                ptr::copy_nonoverlapping(mapped, output.as_mut_ptr() as *mut c_void, staging_buffer.buffer_size as usize);
            }
            vkUnmapMemory(device.handle(), staging_buffer.host_buffer_memory().memory());
            // compute work done
            vkQueueWaitIdle(device.queue().handle());
            CommandDispatch { output: output }
        }
    }
}

pub struct ComputePipeline<
    'instance,
    'device: 'instance,
    'command: 'device,
    'staging: 'command,
    'shader: 'device> {
    handle: VkPipeline,
    layout: VkPipelineLayout,
    descriptor_set: VkDescriptorSet,
    command_buffer: VkCommandBuffer,
    fence: VkFence,
    staging_buffer: &'staging StagingBuffer<'instance, 'device, 'command>,
    shader_module: &'shader ShaderModule<'instance, 'device>,
}

impl<'instance, 'device, 'command, 'staging, 'shader>
ComputePipeline<'instance, 'device, 'command, 'staging, 'shader> {
    pub fn new(
        staging_buffer: &'staging StagingBuffer<'instance, 'device, 'command>,
        shader_module: &'shader ShaderModule<'instance, 'device>) -> Self {
        let command_pool = staging_buffer.command_pool();
        let device = command_pool.device();
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
                let binding = VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT as u32,
                    0
                );
                let create_info = VkDescriptorSetLayoutCreateInfo::new(1, &binding);
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
                let buffer_info = VkDescriptorBufferInfo::new(staging_buffer.device_buffer_memory().buffer(), 0, VK_WHOLE_SIZE);
                let write_set = VkWriteDescriptorSet::new(descriptor_set, VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 0, &buffer_info);
                vkUpdateDescriptorSets(device.handle(), 1, &write_set, 0, ptr::null());
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
                #[repr(C)]
                struct SpecializationData {
                    element_count: u32,
                }
                let data = SpecializationData { element_count: staging_buffer.buffer_element_count as u32 };
                let entry = VkSpecializationMapEntry::new(0, 0, mem::size_of::<u32>());
                let spec_info = VkSpecializationInfo::new(
                    1,
                    &entry,
                    mem::size_of::<SpecializationData>(),
                    &data as *const _ as *const c_void
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
            ComputePipeline {
                handle: compute_pipeline,
                layout: pipeline_layout,
                descriptor_set: descriptor_set,
                shader_module: shader_module,
                command_buffer: command_buffer,
                fence: fence,
                staging_buffer: staging_buffer,
            }
        }
    }

    pub fn staging_buffer(&self) -> &StagingBuffer {
        self.staging_buffer
    }
}

impl<'instance, 'device, 'command, 'staging, 'shader> Drop for ComputePipeline<'instance, 'device, 'command, 'staging, 'shader> {
    fn drop(&mut self) {
        println!("Drop ComputePipeline")
    }
}

pub struct StagingBuffer<'a, 'b: 'a, 'c: 'b> {
    buffer_element_count: usize,
    buffer_size: VkDeviceSize,
    device_buffer_memory: BufferMemory<'a, 'b>,
    host_buffer_memory: BufferMemory<'a, 'b>,
    command_pool: &'c CommandPool<'a, 'b>,
}

impl<'a, 'b, 'c> StagingBuffer<'a, 'b, 'c> {
    pub fn new(command_pool: &'c CommandPool<'a, 'b>) -> Self {
        let device = command_pool.device();
        unsafe {
            println!("device: {:?}, command pool: {:?}", device.handle(), command_pool.handle());
            const BUFFER_ELEMENTS: usize = 32;
            let buffer_size = (BUFFER_ELEMENTS * mem::size_of::<u32>()) as VkDeviceSize;
            let mut input: Vec<u32> = Vec::with_capacity(BUFFER_ELEMENTS);
            input.resize(BUFFER_ELEMENTS, 0);
            for (i, v) in input.iter_mut().enumerate() {
                *v = i as u32
            }
            // host buffer
            let host_buffer_memory = BufferMemory::new(
                device,
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                    VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32, 
                VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32,
                buffer_size,
                input.as_mut_ptr() as *mut c_void).unwrap();
            // Flush writes to host visible buffer
            let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
            vkMapMemory(device.handle(), host_buffer_memory.memory(), 0, VK_WHOLE_SIZE, 0, mapped.as_mut_ptr())
                .into_result()
                .unwrap();
            let mapped_memory_range = VkMappedMemoryRange::new(host_buffer_memory.memory(), 0, VK_WHOLE_SIZE);
            vkFlushMappedMemoryRanges(device.handle(), 1, &mapped_memory_range)
                .into_result()
                .unwrap();
            vkUnmapMemory(device.handle(), host_buffer_memory.memory());
            // device buffer
            let device_buffer_memory = BufferMemory::new(
                device,
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                    VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32 |
                    VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
                VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
                buffer_size,
                ptr::null_mut()).unwrap();
            // Copy to staging buffer
            let allocate_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
            let mut copy_command = MaybeUninit::<VkCommandBuffer>::zeroed();
            vkAllocateCommandBuffers(device.handle(), &allocate_info, copy_command.as_mut_ptr())
                .into_result()
                .unwrap();
            let copy_command = copy_command.assume_init();
            let begin_info = VkCommandBufferBeginInfo::new();
            vkBeginCommandBuffer(copy_command, &begin_info)
                .into_result()
                .unwrap();
            let copy_region = VkBufferCopy::new(buffer_size);
            vkCmdCopyBuffer(copy_command, host_buffer_memory.buffer(), device_buffer_memory.buffer(), 1, &copy_region);
            vkEndCommandBuffer(copy_command)
                .into_result()
                .unwrap();
            let submit_info = VkSubmitInfo::with_command_buffer(1, &copy_command);
            let fence_info = VkFenceCreateInfo::new(VK_FLAGS_NONE);
            let mut fence = MaybeUninit::<VkFence>::zeroed();
            vkCreateFence(device.handle(), &fence_info, ptr::null(), fence.as_mut_ptr())
                .into_result()
                .unwrap();
            let fence = fence.assume_init();
            // submit to the queue
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence)
                .into_result()
                .unwrap();
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
            vkDestroyFence(device.handle(), fence, ptr::null());
            vkFreeCommandBuffers(device.handle(), command_pool.handle(), 1, &copy_command);
            StagingBuffer {
                buffer_element_count: BUFFER_ELEMENTS,
                buffer_size: buffer_size,
                device_buffer_memory: device_buffer_memory,
                host_buffer_memory: host_buffer_memory,
                command_pool: command_pool,
            }
        }
    }

    #[inline]
    pub fn command_pool(&self) -> &CommandPool {
        self.command_pool
    }

    #[inline]
    pub fn host_buffer_memory(&self) -> &BufferMemory {
        &self.host_buffer_memory
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &BufferMemory {
        &self.device_buffer_memory
    }
}

impl<'a, 'b, 'c> Drop for StagingBuffer<'a, 'b, 'c> {
    fn drop(&mut self) {
        println!("Drop StagingBuffer")
    }
}
