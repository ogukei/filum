
#[macro_use]
extern crate stala;
use stala::{Context, BufferViewBuilder, PipelineBuilder, DispatchBuilder};

fn main() {
    let context = Context::new().unwrap();
    let view = BufferViewBuilder::new(&context)
        .layout(
            bindings!(
                binding_array!(u32, 100), 
                binding_array!(u32, 6),
            )
        )
        .build()
        .unwrap();
    let pipeline = PipelineBuilder::new(view.buffer())
        .shader("data/headless.comp.spv")
        .build()
        .unwrap();
    
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let binding = view.second_binding();
    binding.update_array(&v);

    let dispatch = DispatchBuilder::new(&pipeline)
        .workgroup_count(6, 1, 1)
        .build()
        .unwrap();
    dispatch.dispatch();

    binding.fetch_array(&mut v);

    println!("{:?}", v)
}
