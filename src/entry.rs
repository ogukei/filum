

use super::instance::{Instance};
use super::device::{DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

pub fn initialize() {
    let instance = Instance::new().unwrap();
    let device = DeviceBuilder::new(&instance)
        .build()
        .unwrap();
    println!("{:?}", device.physical_device().properties().device_name());
    let command_pool = CommandPool::new(&device).unwrap();
    let shader_module = ShaderModule::new(&device).unwrap();
    let staging_buffer = StagingBuffer::new(&command_pool);
    let compute_pipeline = ComputePipeline::new(&staging_buffer, &shader_module);
    let command_dispatch = CommandDispatch::new(&compute_pipeline);
    println!("{:?}", command_dispatch.output);
}
