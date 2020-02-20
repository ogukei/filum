# filum
Easy GPGPU with Rust and Vulkan.

Provides a simple yet easy-to-use interface to do some computations actually done in parallel. 

## Example

Calculating Fibonacci sequence

```rust
let context = Context::new().unwrap();
let num_elements = 32usize;
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

// outputs
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269]
 ```
 
 ## Features
- High-level interface
- Type-safe in-out buffer access
- Vulkan compute kernel
    - Multiple layout binding support
    - Multiple shader stages support
    - Push constants
    - Specialization constants
 
 ## Another Example
 
 Connected Component Labeling. This is actually an implementation of the algorithm below.
 > A Parallel Approach to Object Identification in Large-scale Images
 >
 > https://www.academia.edu/29842500/
 
 ```rust
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

// outputs
/*
 -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1, 
 -1,   9,   9,  -1,  12,  12,  12,  -1, 
 -1,   9,   9,  -1,  12,  12,  12,  -1, 
  9,   9,   9,  -1,  -1,  -1,  -1,  31, 
 -1,   9,   9,   9,  -1,  -1,  -1,  31, 
 -1,  -1,  -1,  -1,  -1,  45,  45,  -1, 
 -1,  45,  -1,  45,  45,  45,  45,  -1, 
 -1,  45,  45,  45,  -1,  -1,  -1,  -1,
*/
```

## Performance
Connected component labeling 8K Image

GPU computation took `~210ms` including memory transfer operations. 

* OS: Ubuntu 18.04 LTS
* CPU: Intel(R) Core(TM) i7-3930K CPU @ 3.20GHz
* GPU: NVIDIA GeForce RTX 2070

![image](https://gist.githubusercontent.com/ogukei/8fbe74217d57a63d46be9e4bb4cae021/raw/0c972f2d8bc70168530828ecbda24ef7173888ce/ccl.png)

## Runtime Environment
- Requires Vulkan 1.1 Runtime
- Vulkan 1.1 supported Graphics Driver

To compile compute shader GLSL into SPIR-V, we recommend [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/) to compile with.

