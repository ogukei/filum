
use crate::vk::*;
use crate::error::Result;

use std::ptr;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

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
    let device = &devices[0];
    let families = device.queue_family_properties();
    println!("{:?}", families);
}

impl VkPhysicalDeviceProperties {
    fn device_name(&self) -> CString {
        unsafe { CStr::from_ptr(self.deviceName.as_ptr()) }
            .to_owned()
    } 
}
