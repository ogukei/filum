
use stala::{Context, PipelineBuilder};

fn main() {
    let context = Context::new().unwrap();
    let pipeline = PipelineBuilder::new(&context)
        .shader("data/headless.comp.spv")
        .layout_x::<u32>(4096)
        .build()
        .unwrap();
    let mut v = (0..4096).collect::<Vec<_>>();
    pipeline.compute(&mut v);
    dbg!(v);
}
