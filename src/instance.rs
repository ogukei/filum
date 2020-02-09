
use crate::vk::*;
use crate::error::Result;
use crate::error::ErrorCode;

use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::Arc;

#[derive(Debug)]
pub struct Instance {
    handle: VkInstance,
}

impl Instance {
    pub fn new() -> Result<Arc<Instance>> {
        let application_name = CString::new("stala")?;
        let engine_name = CString::new("Stalactite Engine")?;
        let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
        let instance_info = VkInstanceCreateInfo::new(&app_info);
        unsafe {
            let mut handle = MaybeUninit::<VkInstance>::zeroed();
            vkCreateInstance(&instance_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let instance = Instance { handle: handle };
            Ok(Arc::new(instance))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkInstance {
        self.handle
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        log_debug!("Drop Instance");
        unsafe {
            vkDestroyInstance(self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

pub struct PhysicalDevicesBuilder<'a> {
    instance: &'a Arc<Instance>,
}

impl<'a> PhysicalDevicesBuilder<'a> {
    pub fn new(instance: &'a Arc<Instance>) -> Self {
        PhysicalDevicesBuilder { instance: instance }
    }

    pub fn build(self) -> Result<Vec<Arc<PhysicalDevice>>> {
        let instance = self.instance;
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            // obtain count
            vkEnumeratePhysicalDevices(instance.handle, count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(size);
            devices.resize(size, ptr::null_mut());
            vkEnumeratePhysicalDevices(instance.handle, count.as_mut_ptr(), devices.as_mut_ptr())
                .into_result()?;
            let devices: Vec<Arc<PhysicalDevice>> = devices.into_iter()
                .map(|v| PhysicalDevice::new(v, instance))
                .map(|v| Arc::new(v))
                .collect();
            Ok(devices)
        }
    }
}

#[derive(Debug)]
pub struct PhysicalDevice {
    handle: VkPhysicalDevice,
    instance: Arc<Instance>,
}

impl PhysicalDevice {
    pub fn new(device: VkPhysicalDevice, instance: &Arc<Instance>) -> Self {
        PhysicalDevice { handle: device, instance: Arc::clone(instance) }
    }

    #[inline]
    pub fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }

    #[inline]
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
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
