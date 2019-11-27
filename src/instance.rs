
use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use libc::{c_float, c_void};

use std::io::Read;

#[derive(Debug)]
pub struct Instance {
    handle: VkInstance,
}

impl Instance {
    pub fn new() -> Result<Instance> {
        let application_name = CString::new("stala")?;
        let engine_name = CString::new("Stalagmite Engine")?;
        let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
        let instance_info = VkInstanceCreateInfo::new(&app_info);
        unsafe {
            let mut handle = MaybeUninit::<VkInstance>::zeroed();
            vkCreateInstance(&instance_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            Ok(Instance { handle: handle })
        }
    }

    pub fn physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
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
                .map(|v| PhysicalDevice::new(v, self))
                .collect();
            Ok(devices)
        }
    }

    #[inline]
    pub fn handle(&self) -> VkInstance {
        self.handle
    }
}

#[derive(Debug)]
pub struct PhysicalDevice<'a> {
    handle: VkPhysicalDevice,
    instance: &'a Instance,
}

impl<'a> PhysicalDevice<'a> {
    pub fn new(device: VkPhysicalDevice, instance: &'a Instance) -> Self {
        PhysicalDevice { handle: device, instance: instance }
    }

    #[inline]
    pub fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }

    pub fn properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties>::zeroed();
            vkGetPhysicalDeviceProperties(self.handle, properties.as_mut_ptr());
            properties.assume_init()
        }
    }

    pub fn queue_families(&self) -> Result<Vec<QueueFamily>> {
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

pub struct QueueFamily {
    index: usize,
    property: VkQueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(index: usize, property: VkQueueFamilyProperties) -> Self {
        QueueFamily { index: index, property: property }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn queue_count(&self) -> u32 {
        self.property.queueCount
    }

    #[inline]
    pub fn is_compute(&self) -> bool {
        self.property.has_compute_queue_bit()
    }
}
