
use super::context::{Context, PipelineBuilder};

pub fn initialize() {
    let context = Context::new().unwrap();
    let pipeline = PipelineBuilder::new(&context)
        .shader("data/headless.comp.spv")
        .layout::<u32>(30)
        .build()
        .unwrap();
    pipeline.compute(&mut (0..30).collect());
    pipeline.compute(&mut (0..30).collect());
}
