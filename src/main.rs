
#[macro_use]
extern crate filum;
use filum::{Context, BufferViewBuilder, PipelineBuilder, DispatchBuilder};

fn main() {
    fibonacci(32);
    connected_component_labeling();
}

fn fibonacci(num_elements: usize) {
    let context = Context::new().unwrap();
    // layout memory
    let view = BufferViewBuilder::new(&context)
        .bind_array::<u32>(num_elements)
        .build()
        .unwrap();
    // prepare compute shader
    let pipeline = PipelineBuilder::new(view.buffer())
        .shader("data/fibonacci.comp.spv")
        .specialization(constants!(num_elements as u32))
        .build()
        .unwrap();
    // send data to GPU device
    let mut v: Vec<u32> = (0..num_elements as u32)
        .collect();
    let binding = view.binding();
    binding.update_array_copying(&v);
    // execute computation
    pipeline.dispatch(num_elements);
    // receive data from GPU device
    binding.fetch_array_copying(&mut v);
    println!("{:?}", v);
}

// an implementation of
// A Parallel Approach to Object Identification in Large-scale Images
// @see https://www.academia.edu/29842500/
fn connected_component_labeling() {
    fn dump(v: &[i32], dim_x: usize) {
        let lines: Vec<String> = v.iter()
            .map(|v| format!("{:3}, ", v))
            .collect();
        let new_line = "\n".to_string();
        let lines: String = lines.chunks(dim_x)
            .flat_map(|chunk| chunk.iter().chain(std::iter::once(&new_line)))
            .cloned()
            .collect();
        println!("{}", lines);
    }
    let dim = (8usize, 8usize);
    let table: Vec<i32> = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 1, 0,
        0, 1, 1, 0, 1, 1, 1, 0,
        1, 1, 1, 0, 0, 0, 0, 1,
        0, 1, 1, 1, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 1, 1, 0,
        0, 1, 0, 1, 1, 1, 1, 0,
        0, 1, 1, 1, 0, 0, 0, 0,
    ];
    let mut table: Vec<i32> = table.into_iter()
        .enumerate()
        .map(|(i, v)| if v == 0 { -1i32 } else { i as i32 })
        .collect();
    let len = table.len();
    assert_eq!(len, dim.0 * dim.1);
    assert!(dim.0.is_power_of_two());
    let context = Context::new().unwrap();
    let buffer_view = BufferViewBuilder::new(&context)
        .bind_array::<i32>(len)
        .build()
        .unwrap();
    let buffer = buffer_view.buffer();
    let column = PipelineBuilder::new(buffer)
        .shader("data/column.comp.spv")
        .specialization(constants!(dim.0 as u32, dim.1 as u32))
        .build()
        .unwrap();
    let merge = PipelineBuilder::new(buffer)
        .shader("data/merge.comp.spv")
        .specialization(constants!(dim.0 as u32, dim.1 as u32))
        .build()
        .unwrap();
    let relabel = PipelineBuilder::new(buffer)
        .shader("data/relabel.comp.spv")
        .build()
        .unwrap();
    let binding = buffer_view.binding();
    binding.update_array_copying(&table);
    // column
    column.dispatch(dim.0);
    // merge
    {
        let mut step_index = 0;
        let mut n = dim.0 >> 1;
        while n != 0 {
            println!("n {}, si {}", n, step_index);
            let dispatch = DispatchBuilder::new(&merge)
                .workgroup_count(n, 1, 1)
                .push_constants(constants!(step_index as u32))
                .build()
                .unwrap();
            dispatch.dispatch();
            n = n >> 1;
            step_index += 1;
        }
    }
    // relabel
    relabel.dispatch(len);
    binding.fetch_array_copying(&mut table);
    // output
    dump(&table, dim.0);
}
