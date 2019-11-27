
use super::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice};

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};

use std::io::Read;

pub struct Device<'a> {
    handle: VkDevice,
    queue: Queue,
    physical_device: PhysicalDevice,
    instance: &'a Instance,
}

impl<'a> Device<'a> {
    #[inline]
    pub fn handle(&self) -> VkDevice {
        self.handle
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline]
    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.physical_device
    }

    pub fn create_command_pool(&self) -> Result<VkCommandPool> {
        unsafe {
            let mut pool = MaybeUninit::<VkCommandPool>::zeroed();
            let info = VkCommandPoolCreateInfo::new(self.queue.family.index() as u32);
            vkCreateCommandPool(self.handle, &info, ptr::null(), pool.as_mut_ptr())
                .into_result()?;
            Ok(pool.assume_init())
        }
    }

    pub fn create_buffer(
        &self,
        usage: VkBufferUsageFlags, 
        memory_property_flags: VkMemoryPropertyFlags, 
        size: VkDeviceSize,
        data: *mut c_void) -> Result<(VkBuffer, VkDeviceMemory)> {
        unsafe {
            // creates buffer
            let mut buffer = MaybeUninit::<VkBuffer>::zeroed();
            let buffer_create_info = VkBufferCreateInfo::new(size, usage, VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);
            vkCreateBuffer(self.handle, &buffer_create_info, ptr::null(), buffer.as_mut_ptr())
                .into_result()
                .unwrap();
            let buffer = buffer.assume_init();
            // physical memory properties
            let mut memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::zeroed();
            vkGetPhysicalDeviceMemoryProperties(self.physical_device().handle(), memory_properties.as_mut_ptr());
            let memory_properties = memory_properties.assume_init();
            // requirements
            let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
            vkGetBufferMemoryRequirements(self.handle, buffer, requirements.as_mut_ptr());
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
            vkAllocateMemory(self.handle, &allocate_info, ptr::null(), memory.as_mut_ptr())
                .into_result()
                .unwrap();
            let memory = memory.assume_init();
            // maps memory if needed
            if data != ptr::null_mut() {
                let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
                vkMapMemory(self.handle, memory, 0, size, 0, mapped.as_mut_ptr())
                    .into_result()
                    .unwrap();
                let mapped = mapped.assume_init();
                ptr::copy_nonoverlapping(data as *mut u8, mapped as *mut u8, size as usize);
                vkUnmapMemory(self.handle, memory);
            }
            // binding
            vkBindBufferMemory(self.handle, buffer, memory, 0)
                .into_result()
                .unwrap();
            Ok((buffer, memory))
        }
    }
}

pub struct Queue {
    handle: VkQueue,
    family: QueueFamily,
}

impl Queue {
    pub fn new(handle: VkQueue, family: QueueFamily) -> Self {
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
    instance: &'a Instance,
}

impl<'a> DeviceBuilder<'a> {
    pub fn new(instance: &'a Instance) -> Self {
        DeviceBuilder { instance }
    }

    pub fn build(self) -> Result<Device<'a>> {
        let devices = self.instance.physical_devices()?;
        let device = devices.first()
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
            Ok(Device {
                handle: handle,
                queue: Queue::new(queue.assume_init(), family),
                physical_device: *device,
                instance: self.instance
            })
        }
    }
}

pub struct ShaderModule {
    handle: VkShaderModule,
}

impl ShaderModule {
    pub fn new(device: &Device) -> std::io::Result<Self> {
        let mut file = std::fs::File::open("data/headless.comp.spv")?;
        let mut buffer = Vec::<u8>::new();
        let bytes = file.read_to_end(&mut buffer)?;
        assert!(bytes > 0);
        assert_eq!(bytes % 4, 0);
        unsafe {
            let mut shader_module = MaybeUninit::<VkShaderModule>::zeroed();
            let create_info = VkShaderModuleCreateInfo::new(bytes, std::mem::transmute(buffer.as_mut_ptr()));
                vkCreateShaderModule(device.handle, &create_info, ptr::null(), shader_module.as_mut_ptr())
                    .into_result()
                    .unwrap();
            let shader_module = shader_module.assume_init();
            Ok(ShaderModule {
                handle: shader_module,
            })
        }
    }

    #[inline]
    pub fn handle(&self) -> VkShaderModule {
        self.handle
    }
}
