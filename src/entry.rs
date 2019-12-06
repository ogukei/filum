
use super::context::{Context, PipelineBuilder};

pub fn initialize() {
    let context = Context::new().unwrap();
    let pipeline = PipelineBuilder::new(&context)
        .shader("data/headless.comp.spv")
        .layout_x::<u32>(4096)
        .build()
        .unwrap();
    pipeline.compute(&mut (0..4096).collect::<Vec<_>>());
}
