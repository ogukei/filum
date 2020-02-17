
#[macro_use]
extern crate stala;
use stala::{Context, BufferViewBuilder, PipelineBuilder, DispatchBuilder};

fn main() {

    ccl();

}

fn ccl() {
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
    let v: Vec<i32> = table.into_iter()
        .enumerate()
        .map(|(i, v)| if v == 0 { -1i32 } else { i as i32 })
        .collect();
    union_find(v, 8);
}

fn union_find(table: Vec<i32>, dim: usize) {
    fn dump(v: &[i32]) {
        let lines: Vec<String> = v.iter()
            .map(|v| format!("{:3}, ", v))
            .collect();
        let new_line = "\n".to_string();
        let hoge: String = lines.chunks(8)
            .flat_map(|chunk| chunk.iter().chain(std::iter::once(&new_line)))
            .cloned()
            .collect();
        println!("{}", hoge);
    }

    let mut table = table;
    let len = table.len();
    assert_eq!(len, dim * dim);
    // power of two
    assert_eq!((dim & (dim - 1)), 0);
    
    let context = Context::new().unwrap();
    let view = BufferViewBuilder::new(&context)
        .bind_array::<i32>(len)
        .build()
        .unwrap();
    let column = PipelineBuilder::new(view.buffer())
        .shader("data/column.comp.spv")
        .specialization(constants!(dim as u32, dim as u32))
        .build()
        .unwrap();
    let merge = PipelineBuilder::new(view.buffer())
        .shader("data/merge.comp.spv")
        .specialization(constants!(dim as u32, dim as u32))
        .build()
        .unwrap();
    let relabel = PipelineBuilder::new(view.buffer())
        .shader("data/relabel.comp.spv")
        .build()
        .unwrap();
    view.binding().update_array_copying(&table);

    column.dispatch(dim);

    let mut step_index = 0;
    let mut n = dim >> 1;
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
    relabel.dispatch(len);
    view.binding().fetch_array_copying(&mut table);
    dump(&table);
}

fn fibo(num_elements: usize) {
    let context = Context::new().unwrap();
    let view = BufferViewBuilder::new(&context)
        .bind_array::<u32>(num_elements)
        .build()
        .unwrap();
    let pipeline = PipelineBuilder::new(view.buffer())
        .shader("data/headless.comp.spv")
        .specialization(constants!(num_elements as u32))
        .build()
        .unwrap();
    
    let mut v = (0..num_elements as u32).collect::<Vec<_>>();
    let binding = view.binding();
    binding.update_array_copying(&v);

    let dispatch = DispatchBuilder::new(&pipeline)
        .workgroup_count(num_elements, 1, 1)
        .push_constants(constants!(100u32, -42i32))
        .build()
        .unwrap();
    dispatch.dispatch();

    binding.fetch_array_copying(&mut v);

    println!("{:?}", v)
}