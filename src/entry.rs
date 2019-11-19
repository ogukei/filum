
use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;

use std::ptr;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float};

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
    println!("device: {:?}", device.handle);
}

struct Device {
    handle: VkDevice,
    queue: Queue,
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
    pub fn new() -> DeviceBuilder { DeviceBuilder {} }

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
