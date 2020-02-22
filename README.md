# filum
Easy GPGPU with Rust and Vulkan.

Provides a simple yet easy-to-use interface to do some computations actually done in parallel. 

## Example

Calculating Fibonacci sequence

```rust
extern crate filum;
use filum::{Context, BufferViewBuilder, PipelineBuilder};

fn main() {
    // setup data to calculate fibonacci sequence
    let num_elements = 32usize;
    let mut v: Vec<u32> = (0..num_elements as u32).collect();
    // filum automatically selects one from available GPUs. 
    // context contains information of the GPU.
    let context = Context::new().unwrap();
    // allocates contiguous memory of u32 with the number of `num_elements`.
    let buffer_view = BufferViewBuilder::new(&context)
        .bind_array::<u32>(num_elements)
        .build()
        .unwrap();
    // loads a compute shader from the specified file path
    // and associates it to the buffer.
    let pipeline = PipelineBuilder::new(buffer_view.buffer())
        .shader("data/fibonacci.comp.spv")
        .build()
        .unwrap();
    // in order to transfer our data to the GPU, 
    // gets a reference which corresponds to the binding point that
    // indicates the location of the array of u32 stored on the GPU.
    let binding = buffer_view.binding();
    // sends data to the GPU
    binding.update_array_copying(&v);
    // runs the computation specifying how many invocations of 
    // the shader performed.
    pipeline.dispatch(num_elements);
    // retrieves back data from the GPU
    binding.fetch_array_copying(&mut v);
    println!("{:?}", v);
}
// outputs
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269]
```

## Features
- High-level interface
- Lightweight
- Type-safe in-out buffer access
- Cargo support
- Vulkan compute shaders
    - Multiple binding points
    - Multiple compute shaders
    - Push constants
    - Specialization constants

## Quickstart

Make sure `vulkaninfo` command runs on your system and Vulkan version matches `1.1.x` or above.

Running an example project
```
git clone https://github.com/ogukei/filum-example
cd filum-example/fibonacci
cargo run --release
```

Update your Cargo.toml if you want starting from scratch.
```
[dependencies]
filum = "*"
```

## Another Example

Connected Component Labeling, implemented the following algorithm.

> A Parallel Approach to Object Identification in Large-scale Images \
> https://www.academia.edu/29842500/

The actual implementation is available at [filum-example](https://github.com/ogukei/filum-example)

```rust
#[macro_use]
extern crate filum;
use filum::{Context, BufferViewBuilder, PipelineBuilder, DispatchBuilder};

fn main() {
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

Connected Component Labeling 8K Image

GPU computation took `~210ms` including memory transfer operations with the following environment.

* OS: Ubuntu 18.04 LTS
* CPU: Intel(R) Core(TM) i7-3930K CPU @ 3.20GHz
* GPU: NVIDIA GeForce RTX 2070

Result (resized) |
:-:|
![image](https://gist.githubusercontent.com/ogukei/8fbe74217d57a63d46be9e4bb4cae021/raw/0c972f2d8bc70168530828ecbda24ef7173888ce/ccl.png) |

_The Vulkan logo is a trademark of the Khronos Group Inc._

## Runtime Environment
- Requires Vulkan 1.1 Runtime
- Vulkan 1.1 capable graphics drivers

To compile compute shader GLSL into SPIR-V, we recommend [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/) to compile with.

## Getting Started

Install Vulkan 1.1 compatible OS. Ubuntu 18.04 LTS is highly recommended since its graphics driver installation is pretty easy.

For Ubuntu 18.04 LTS users, proceed the following steps. The other OS users, see the information at https://www.lunarg.com/vulkan-sdk/ and proceed to the step 3 once your Vulkan setup is done.

1. Run the following command if you don't have Vulkan 1.1 compatible graphics driver yet

Please ensure you have Vulkan 1.1 capable graphics cards checking the following pages.

For NVIDIA graphics cards users \
https://developer.nvidia.com/vulkan-driver

For AMD graphics cards users \
https://gpuopen.com/gaming-product/vulkan/

```
ubuntu-drivers list
ubuntu-drivers install <recommended-version>
```

Please restart your system to complete the driver installation.

2. Run the following command if you don't have Vulkan SDK installed
```
sudo apt update
sudo apt install vulkan-sdk
```

3. Run the following command to check your setup is done properly
```
vulkaninfo
```

4. Clone our example project
```
git clone https://github.com/ogukei/filum-example
```

5. Run an example project
```
cd filum-example
cd fibonacci
cargo run --release
```

6. Compile GLSL compute shaders using `glslc`
```
cd filum-example
cd fibonacci
make -j
```
