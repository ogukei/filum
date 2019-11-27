

use super::instance::{Instance};
use super::device::{DeviceBuilder};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

pub fn initialize() {
    let instance = Instance::new().unwrap();
    let device = DeviceBuilder::new(&instance)
        .build()
        .unwrap();
    println!("{:?}", device.physical_device().properties().device_name());

    let command_pool = device.create_command_pool().unwrap();
    let staging_buffer = StagingBuffer::new(&device, command_pool);
    let compute_pipeline = ComputePipeline::new(&device, &staging_buffer, command_pool);
    let command_dispatch = CommandDispatch::new(&device, &staging_buffer, &compute_pipeline);
    println!("{:?}", command_dispatch.output);
}
