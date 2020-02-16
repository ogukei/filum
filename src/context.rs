
use super::instance::{Instance};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;

pub struct Context {
    instance: Arc<Instance>,
    device: Arc<Device>,
    command_pool: Arc<CommandPool>,
}

impl Context {
    pub fn new() -> Result<Arc<Self>> {
        let instance = Instance::new()?;
        let device = DeviceBuilder::new(&instance).build()?;
        let command_pool = CommandPool::new(&device)?;
        log_debug!("GPU: {:?}", device.physical_device().properties().device_name());
        let context = Context { 
            instance: instance,
            device: device,
            command_pool: command_pool,
        };
        Ok(Arc::new(context))
    }

    #[inline]
    pub(crate) fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    #[inline]
    pub(crate) fn device(&self) -> &Arc<Device> {
        &self.device
    }

    #[inline]
    pub(crate) fn command_pool(&self) -> &Arc<CommandPool> {
        &self.command_pool
    }
}
