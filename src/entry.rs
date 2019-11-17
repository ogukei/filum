
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
            let mut handle = MaybeUninit::<VkInstance>::uninit();
            vkCreateInstance(&instance_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            Ok(Instance { handle: handle })
        }
    }

    fn physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::uninit();
            // obtain count
            vkEnumeratePhysicalDevices(self.handle, count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(size);
            devices.resize(size, ptr::null_mut());
            vkEnumeratePhysicalDevices(self.handle, count.as_mut_ptr(), devices.as_mut_ptr())
                .into_result()?;
            let devices: Vec<_> = devices.into_iter()
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
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties>::uninit();
            vkGetPhysicalDeviceProperties(self.handle, properties.as_mut_ptr());
            properties.assume_init()
        }
    }

    fn queue_family_properties(&self) -> Result<Vec<VkQueueFamilyProperties>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::uninit();
            // obtain count
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), ptr::null_mut());
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut families: Vec<VkQueueFamilyProperties> = Vec::with_capacity(size);
            families.resize(size, std::mem::zeroed());
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), families.as_mut_ptr());
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
    println!("{:?}", device.handle);
}

struct Device {
    handle: VkDevice
}

struct DeviceBuilder {

}

impl DeviceBuilder {
    pub fn new() -> DeviceBuilder { DeviceBuilder {} }

    pub fn build(self, devices: &Vec<PhysicalDevice>) -> Result<Device> {
        let device = devices.first()
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let families = device.queue_family_properties()?;
        // iterate through compute family candidates keeping the indices
        let compute_families: Vec<_> = families.iter()
            .enumerate()
            .filter(|(_, family)| family.has_compute_queue_bit())
            .collect();
        let compute_family = compute_families.first()
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let priority: c_float = unsafe { std::mem::zeroed() };
        let family_index = compute_family.0 as u32;
        let queue_create_info = VkDeviceQueueCreateInfo::new(family_index, 1, &priority);
        let device_create_info = VkDeviceCreateInfo::new(1, &queue_create_info);
        unsafe {
            let mut handle = MaybeUninit::<VkDevice>::uninit();
            vkCreateDevice(device.handle, &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            Ok(Device { handle: handle.assume_init() })
        }
    }
}

impl VkPhysicalDeviceProperties {
    fn device_name(&self) -> CString {
        unsafe { CStr::from_ptr(self.deviceName.as_ptr()) }
            .to_owned()
    } 
}
