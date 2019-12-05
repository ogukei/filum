

use super::instance::{Instance};
use super::device::{DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

use super::context::{Context, PipelineBuilder};

pub fn initialize() {
    let context = Context::new().unwrap();
    let pipeline = PipelineBuilder::new(&context)
        .shader("data/headless.comp.spv")
        .layout::<u32>(24)
        .build()
        .unwrap();
    pipeline.compute(&vec![1, 2, 3, 4]);
}
