
use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};

#[derive(Debug)]
struct Instance {
    handle: VkInstance,
}

impl Instance {
    fn new() -> Result<Instance> {
        let application_name = CString::new("stala")?;
        let engine_name = CString::new("Stalagmite Engine")?;
        let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
        let instance_info = VkInstanceCreateInfo::new(app_info);
        unsafe {
            let mut handle = MaybeUninit::<VkInstance>::zeroed();
            vkCreateInstance(&instance_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            Ok(Instance { handle: handle })
        }
    }

    fn physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            // obtain count
            vkEnumeratePhysicalDevices(self.handle, count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(size);
            devices.resize(size, ptr::null_mut());
            vkEnumeratePhysicalDevices(self.handle, count.as_mut_ptr(), devices.as_mut_ptr())
                .into_result()?;
            let devices: Vec<PhysicalDevice> = devices.into_iter()
                .map(|e| PhysicalDevice::new(e))
                .collect();
            Ok(devices)
        }
    }
}

#[derive(Debug)]
struct PhysicalDevice {
    handle: VkPhysicalDevice,
}

impl PhysicalDevice {
    fn new(device: VkPhysicalDevice) -> Self {
        PhysicalDevice { handle: device }
    }

    fn properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties>::zeroed();
            vkGetPhysicalDeviceProperties(self.handle, properties.as_mut_ptr());
            properties.assume_init()
        }
    }

    fn queue_families(&self) -> Result<Vec<QueueFamily>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            // obtain count
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), ptr::null_mut());
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut families: Vec<VkQueueFamilyProperties> = Vec::with_capacity(size);
            families.resize(size, std::mem::zeroed());
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), families.as_mut_ptr());
            let families: Vec<QueueFamily> = families.into_iter()
                .enumerate()
                .map(|(i,v)| QueueFamily::new(i, v))
                .collect();
            Ok(families)
        }
    }
}

pub fn initialize() {
    let instance = Instance::new().unwrap();
    println!("{:?}", instance);

    let devices = instance.physical_devices().unwrap();
    println!("{:?}", devices);

    let properties: Vec<_> = devices.iter()
        .map(|v|v.properties())
        .collect();

    for property in properties {
        println!("{:?}", property.device_name());
    }

    let device = DeviceBuilder::new()
        .build(&devices)
        .unwrap();

    let command_pool = device.create_command_pool().unwrap();
    println!("device: {:?}, command pool: {:?}", device.handle, command_pool);
    let transfer_src = VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
    let transfer_dst = VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
    let host_visible = VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32;
    const BUFFER_ELEMENTS: usize = 32;
    let buffer_size = (BUFFER_ELEMENTS * mem::size_of::<u32>()) as VkDeviceSize;

    let mut input: Vec<u32> = Vec::with_capacity(BUFFER_ELEMENTS);
    let mut output: Vec<u32> = Vec::with_capacity(BUFFER_ELEMENTS);
    input.resize(BUFFER_ELEMENTS, 0);
    output.resize(BUFFER_ELEMENTS, 0);
    let (host_buffer) = device.create_buffer(
        transfer_src | transfer_dst, 
        host_visible,
        buffer_size,
        input.as_mut_ptr() as *mut c_void).unwrap();
    println!("buffer: {:?}", host_buffer);
}

struct Device {
    handle: VkDevice,
    queue: Queue,
    physical_device_handle: VkPhysicalDevice,
}

impl Device {
    fn create_command_pool(&self) -> Result<VkCommandPool> {
        unsafe {
            let mut pool = MaybeUninit::<VkCommandPool>::zeroed();
            let info = VkCommandPoolCreateInfo::new(self.queue.family.index() as u32);
            vkCreateCommandPool(self.handle, &info, ptr::null(), pool.as_mut_ptr())
                .into_result()?;
            Ok(pool.assume_init())
        }
    }

    fn create_buffer(
        &self,
        usage: VkBufferUsageFlags, 
        memory_property_flags: VkMemoryPropertyFlags, 
        size: VkDeviceSize,
        data: *mut c_void) -> Result<(VkBuffer)> {
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
            vkGetPhysicalDeviceMemoryProperties(self.physical_device_handle, memory_properties.as_mut_ptr());
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
            Ok(buffer)
        }
    }
}

struct QueueFamily {
    index: usize,
    property: VkQueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(index: usize, property: VkQueueFamilyProperties) -> Self {
        QueueFamily { index: index, property: property }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn queue_count(&self) -> u32 {
        self.property.queueCount
    }

    pub fn is_compute(&self) -> bool {
        self.property.has_compute_queue_bit()
    }
}

struct Queue {
    handle: VkQueue,
    family: QueueFamily,
}

impl Queue {
    pub fn new(handle: VkQueue, family: QueueFamily) -> Self {
        Queue { handle: handle, family: family }
    }
}

struct DeviceBuilder {

}

impl DeviceBuilder {
    pub fn new() -> Self { DeviceBuilder {} }

    pub fn build(self, devices: &Vec<PhysicalDevice>) -> Result<Device> {
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
            vkCreateDevice(device.handle, &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            // queues
            let mut queue = MaybeUninit::<VkQueue>::zeroed();
            vkGetDeviceQueue(handle, family_index, 0, queue.as_mut_ptr());
            Ok(Device {
                handle: handle,
                queue: Queue::new(queue.assume_init(), family),
                physical_device_handle: device.handle,
            })
        }
    }
}

impl VkPhysicalDeviceProperties {
    fn device_name(&self) -> CString {
        unsafe { CStr::from_ptr(self.deviceName.as_ptr()) }
            .to_owned()
    } 
}
