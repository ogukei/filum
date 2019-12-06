

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
    pub fn new(compute_pipeline: &Arc<ComputePipeline>, workgroup_size: WorkgroupSize) -> Arc<Self> {
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
            // copy to staging buffer
            let copy_region = VkBufferCopy::new(staging_buffer.buffer_size);
            vkCmdCopyBuffer(
                command_buffer, 
                staging_buffer.host_buffer_memory().buffer(), 
                staging_buffer.device_buffer_memory().buffer(), 
                1,
                &copy_region
            );
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
            vkCmdDispatch(command_buffer, workgroup_size.x, workgroup_size.y, workgroup_size.z);
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
        println!("Drop CommandDispatch");
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

pub struct WorkgroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
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
    pub fn new(staging_buffer: &Arc<StagingBuffer>, shader_module: &Arc<ShaderModule>) -> Arc<Self> {
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
        println!("Drop ComputePipeline");
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

pub struct BufferMemoryLayout<T: Sized> {
    buffer_size: VkDeviceSize,
    element_count: usize,
    value: PhantomData<T>
}

impl<T> BufferMemoryLayout<T> where T: Sized {
    pub fn new(element_count: usize) -> Self {
        let buffer_size = (element_count * mem::size_of::<T>()) as VkDeviceSize;
        BufferMemoryLayout {
            buffer_size,
            element_count,
            value: PhantomData
        }
    }

    pub fn element_count(&self) -> usize {
       self.element_count 
    }

    pub fn buffer_size(&self) -> VkDeviceSize {
        self.buffer_size
    }
}

pub struct StagingBuffer {
    buffer_element_count: usize,
    buffer_size: VkDeviceSize,
    device_buffer_memory: Arc<BufferMemory>,
    host_buffer_memory: Arc<BufferMemory>,
    command_pool: Arc<CommandPool>,
}

impl StagingBuffer {
    pub fn new<T: Sized>(command_pool: &Arc<CommandPool>, layout: &BufferMemoryLayout<T>) -> Arc<Self> {
        let device = command_pool.device();
        let buffer_size = layout.buffer_size();
        let element_count = layout.element_count();
        // host buffer
        let host_buffer_memory = BufferMemory::new(
            device,
            VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32, 
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32,
            buffer_size,
            ptr::null_mut()).unwrap();
        // device buffer
        let device_buffer_memory = BufferMemory::new(
            device,
            VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32 | 
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32 |
                VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as u32,
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            buffer_size,
            ptr::null_mut()).unwrap();
        let staging_buffer = StagingBuffer {
            buffer_element_count: element_count,
            buffer_size: buffer_size,
            device_buffer_memory: device_buffer_memory,
            host_buffer_memory: host_buffer_memory,
            command_pool: Arc::clone(command_pool),
        };
        Arc::new(staging_buffer)
    }

    pub fn write_host_memory<T: Sized>(&self, layout: &BufferMemoryLayout<T>, vec: &mut [T]) {
        assert_eq!(vec.len(), layout.element_count());
        assert_eq!(std::mem::size_of::<T>() * vec.len(), layout.buffer_size() as usize);
        // writes memory
        self.host_buffer_memory.write_memory(vec.as_mut_ptr() as *mut c_void);
    }

    pub fn read_host_memory<T: Sized>(&self, layout: &BufferMemoryLayout<T>, vec: &mut [T]) {
        assert_eq!(vec.len(), layout.element_count());
        assert_eq!(std::mem::size_of::<T>() * vec.len(), layout.buffer_size() as usize);
        // reads memory
        self.host_buffer_memory.read_memory(vec.as_mut_ptr() as *mut c_void);
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
}

impl Drop for StagingBuffer {
    fn drop(&mut self) {
        println!("Drop StagingBuffer");
    }
}
