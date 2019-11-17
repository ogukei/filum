
use crate::vk::*;
use crate::error::Result;
use crate::error::Error;

use std::ptr;
use std::ffi::CString;
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
            let result = vkCreateInstance(&instance_info, ptr::null(), instance.as_mut_ptr());
            if result == VkResult::VK_SUCCESS {
                let instance = instance.assume_init();
                Ok(Instance { instance: instance })
            } else {
                Err(Error::from(result))
            }
        }
    }
}

pub fn initialize() {
    let instance = Instance::new();
    println!("{:?}", instance);
}
