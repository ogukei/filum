
use super::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;

pub struct Device {
    handle: VkDevice,
    queue: Queue,
    physical_device: Arc<PhysicalDevice>,
}

impl Device {
    #[inline]
    pub fn handle(&self) -> VkDevice {
        self.handle
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline]
    pub fn physical_device(&self) -> &Arc<PhysicalDevice> {
        &self.physical_device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        log_debug!("Drop Device");
        unsafe {
            vkDestroyDevice(self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

pub struct BufferMemory {
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    device: Arc<Device>,
    whole_size: VkDeviceSize,
}

impl BufferMemory {
    pub fn new(device: &Arc<Device>, 
        usage: VkBufferUsageFlags, 
        memory_property_flags: VkMemoryPropertyFlags, 
        size: VkDeviceSize) -> Result<Arc<Self>> {
        unsafe {
            // creates buffer
            let mut buffer = MaybeUninit::<VkBuffer>::zeroed();
            let buffer_create_info = VkBufferCreateInfo::new(size, usage, VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);
            vkCreateBuffer(device.handle(), &buffer_create_info, ptr::null(), buffer.as_mut_ptr())
                .into_result()
                .unwrap();
            let buffer = buffer.assume_init();
            // physical memory properties
            let mut memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::zeroed();
            vkGetPhysicalDeviceMemoryProperties(device.physical_device().handle(), memory_properties.as_mut_ptr());
            let memory_properties = memory_properties.assume_init();
            // requirements
            let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
            vkGetBufferMemoryRequirements(device.handle(), buffer, requirements.as_mut_ptr());
            let requirements = requirements.assume_init();
            // find a memory type index that fits the properties
            let memory_type_bits = requirements.memoryTypeBits;
            let memory_type_index = memory_properties.memoryTypes.iter()
                .enumerate()
                .filter(|(i,_)| ((memory_type_bits >> i) & 1) == 1)
                .filter(|(_,v)| (v.propertyFlags & memory_property_flags) == memory_property_flags)
                .nth(0)
                .map(|(i,_)| i as u32)
                .ok_or_else(|| ErrorCode::SuitableBufferMemoryTypeNotFound)
                .unwrap();
            // allocation
            let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
            let allocate_info = VkMemoryAllocateInfo::new(requirements.size, memory_type_index);
            vkAllocateMemory(device.handle(), &allocate_info, ptr::null(), memory.as_mut_ptr())
                .into_result()
                .unwrap();
            let memory = memory.assume_init();
            // binding
            vkBindBufferMemory(device.handle(), buffer, memory, 0)
                .into_result()
                .unwrap();
            let buffer_memory = BufferMemory { 
                buffer: buffer,
                memory: memory,
                device: Arc::clone(device),
                whole_size: size,
            };
            Ok(Arc::new(buffer_memory))
        }
    }

    #[inline]
    pub fn buffer(&self) -> VkBuffer {
        self.buffer
    }

    #[inline]
    pub fn memory(&self) -> VkDeviceMemory {
        self.memory
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for BufferMemory {
    fn drop(&mut self) {
        unsafe {
            log_debug!("Drop BufferMemory");
            vkDestroyBuffer(self.device.handle(), self.buffer, ptr::null());
            self.buffer = ptr::null_mut();
            vkFreeMemory(self.device.handle(), self.memory, ptr::null());
            self.memory = ptr::null_mut();
        }
    }
}

struct BufferMemoryMappedRange {
    offset: VkDeviceSize, 
    size: VkDeviceSize,
    mapped: *mut c_void,
    buffer_memory: Arc<BufferMemory>,
}

impl BufferMemoryMappedRange {
    fn new(buffer_memory: &Arc<BufferMemory>, offset: VkDeviceSize, size: VkDeviceSize) -> Arc<Self> {
        let device = buffer_memory.device();
        // Make device writes visible to the host
        let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
        unsafe {
            vkMapMemory(device.handle(), buffer_memory.memory(), offset, size, 0, mapped.as_mut_ptr())
                .into_result()
                .unwrap();
            let mapped = mapped.assume_init();
            let range = BufferMemoryMappedRange {
                offset,
                size,
                mapped,
                buffer_memory: Arc::clone(buffer_memory),
            };
            Arc::new(range)
        }
    } 

    fn invalidate(&self) {
        let device = self.buffer_memory.device();
        let memory = self.buffer_memory.memory();
        let mapped_range = VkMappedMemoryRange::new(memory, self.offset, self.size);
        unsafe {
            vkInvalidateMappedMemoryRanges(device.handle(), 1, &mapped_range)
                .into_result()
                .unwrap();
        }
    }

    fn flush(&self) {
        let device = self.buffer_memory.device();
        let memory = self.buffer_memory.memory();
        let mapped_range = VkMappedMemoryRange::new(memory, self.offset, self.size);
        unsafe {
            vkFlushMappedMemoryRanges(device.handle(), 1, &mapped_range)
                .into_result()
                .unwrap();
        }
    }
}

impl Drop for BufferMemoryMappedRange {
    fn drop(&mut self) {
        unsafe {
            let device = self.buffer_memory.device();
            let memory = self.buffer_memory.memory();
            vkUnmapMemory(device.handle(), memory);
        }
    }
}

pub struct CommandPool {
    handle: VkCommandPool,
    device: Arc<Device>,
}

impl CommandPool {
    pub fn new(device: &Arc<Device>) -> Result<Arc<Self>> {
        unsafe {
            let mut handle = MaybeUninit::<VkCommandPool>::zeroed();
            let info = VkCommandPoolCreateInfo::new(device.queue().family().index() as u32);
            vkCreateCommandPool(device.handle, &info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let command_pool = CommandPool {
                handle: handle,
                device: Arc::clone(device),
            };
            Ok(Arc::new(command_pool))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkCommandPool {
        self.handle
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        log_debug!("Drop CommandPool");
        unsafe {
            vkDestroyCommandPool(self.device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

pub struct Queue {
    handle: VkQueue,
    family: QueueFamily,
}

impl Queue {
    fn new(handle: VkQueue, family: QueueFamily) -> Self {
        Queue { handle: handle, family: family }
    }

    #[inline]
    pub fn handle(&self) -> VkQueue {
        self.handle
    }

    #[inline]
    pub fn family(&self) -> &QueueFamily {
        &self.family
    }
}

pub struct DeviceBuilder<'a> {
    instance: &'a Arc<Instance>,
}

impl<'a> DeviceBuilder<'a> {
    pub fn new(instance: &'a Arc<Instance>) -> Self {
        DeviceBuilder { instance }
    }

    pub fn build(self) -> Result<Arc<Device>> {
        let devices = PhysicalDevicesBuilder::new(self.instance).build()?;
        let device = devices.into_iter()
            .nth(0)
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let families = device.queue_families()?;
        // iterate through compute family candidates keeping the indices
        let compute_families: Vec<_> = families.into_iter()
            .filter(|family| family.is_compute())
            .collect();
        // request single queue
        let family = compute_families.into_iter()
            .nth(0)
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let family_index = family.index() as u32;
        let priority: c_float = 0.0;
        let queue_create_info = VkDeviceQueueCreateInfo::new(family_index, 1, &priority);
        let device_create_info = VkDeviceCreateInfo::new(1, &queue_create_info);
        unsafe {
            let mut handle = MaybeUninit::<VkDevice>::zeroed();
            vkCreateDevice(device.handle(), &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            // queues
            let mut queue = MaybeUninit::<VkQueue>::zeroed();
            vkGetDeviceQueue(handle, family_index, 0, queue.as_mut_ptr());
            let queue = Queue::new(queue.assume_init(), family);
            let device = Device {
                handle: handle,
                queue: queue,
                physical_device: device,
            };
            Ok(Arc::new(device))
        }
    }
}

pub struct ShaderModule {
    handle: VkShaderModule,
    device: Arc<Device>,
}

impl ShaderModule {
    pub fn new<S: Into<String>>(device: &Arc<Device>, filename: S) -> std::io::Result<Arc<Self>> {
        let mut file = std::fs::File::open(filename.into())?;
        let mut buffer = Vec::<u8>::new();
        let bytes = file.read_to_end(&mut buffer)?;
        assert!(bytes > 0);
        assert_eq!(bytes % 4, 0);
        unsafe {
            let mut handle = MaybeUninit::<VkShaderModule>::zeroed();
            let create_info = VkShaderModuleCreateInfo::new(bytes, std::mem::transmute(buffer.as_mut_ptr()));
            vkCreateShaderModule(device.handle, &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
            let handle = handle.assume_init();
            let shader_module = ShaderModule {
                handle: handle,
                device: Arc::clone(device),
            };
            Ok(Arc::new(shader_module))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkShaderModule {
        self.handle
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        log_debug!("Drop ShaderModule");
        unsafe {
            vkDestroyShaderModule(self.device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}
