
use crate::vk::*;
use crate::error::Result;
use crate::error::Error;

use std::ptr;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

#[derive(Debug)]
struct Instance {
    instance: VkInstance,
}

impl Instance {
    fn new() -> Result<Instance> {
        let application_name = CString::new("stala")?;
        let engine_name = CString::new("Stalagmite Engine")?;
        let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
        let instance_info = VkInstanceCreateInfo::new(app_info);
        unsafe {
            let mut instance = MaybeUninit::<VkInstance>::uninit();
            vkCreateInstance(&instance_info, ptr::null(), instance.as_mut_ptr())
                .into_result()?;
            let instance = instance.assume_init();
            Ok(Instance { instance: instance })
        }
    }

    fn physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::uninit();
            // obtain count
            vkEnumeratePhysicalDevices(self.instance, count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(size);
            devices.resize(size, ptr::null_mut());
            vkEnumeratePhysicalDevices(self.instance, count.as_mut_ptr(), devices.as_mut_ptr())
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
    device: VkPhysicalDevice,
}

impl PhysicalDevice {
    fn new(device: VkPhysicalDevice) -> Self {
        PhysicalDevice { device: device }
    }

    fn properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties>::uninit();
            vkGetPhysicalDeviceProperties(self.device, properties.as_mut_ptr());
            properties.assume_init()
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
}

impl VkPhysicalDeviceProperties {
    fn device_name(&self) -> CString {
        unsafe { CStr::from_ptr(self.deviceName.as_ptr()) }
            .to_owned()
    } 
}
