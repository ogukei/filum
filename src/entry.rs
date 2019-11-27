

use super::instance::{Instance};
use super::device::{DeviceBuilder};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

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

    let command_pool = device.create_command_pool().unwrap();
    let staging_buffer = StagingBuffer::new(&device, command_pool);
    let compute_pipeline = ComputePipeline::new(&device, &staging_buffer, command_pool);
    let command_dispatch = CommandDispatch::new(&device, &staging_buffer, &compute_pipeline);
    println!("{:?}", command_dispatch.output);
}
