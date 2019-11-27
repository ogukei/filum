

use super::instance::{Instance};
use super::device::{DeviceBuilder, CommandPool};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

pub fn initialize() {
    let instance = Instance::new().unwrap();
    let device = DeviceBuilder::new(&instance)
        .into_device()
        .unwrap();
    println!("{:?}", device.physical_device().properties().device_name());
    let command_pool = CommandPool::new(&device).unwrap();
    let staging_buffer = StagingBuffer::new(&command_pool);
    let compute_pipeline = ComputePipeline::new(&staging_buffer);
    let command_dispatch = CommandDispatch::new(&compute_pipeline);
    println!("{:?}", command_dispatch.output);
}
