
use super::instance::{Instance};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch, BufferMemoryLayout, WorkgroupSize};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;

pub struct Context {
    instance: Arc<Instance>,
}

impl Context {
    pub fn new() -> Result<Arc<Self>> {
        let instance = Instance::new()?;
        let context = Context { instance: instance };
        Ok(Arc::new(context))
    }

    pub(crate) fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
}

pub struct PipelineLayout<T> where T: Sized + std::fmt::Debug {
    count: usize,
    value: PhantomData<T>,
}

impl<T> PipelineLayout<T> where T: Sized + std::fmt::Debug {
    fn new(count: usize) -> Self {
        PipelineLayout {
            count: count,
            value: PhantomData,
        }
    }
}

pub struct PipelineBuilder<'a, ShaderType, LayoutType, WorkgroupType> {
    shader: ShaderType,
    layout: LayoutType,
    workgroup: WorkgroupType,
    context: &'a Arc<Context>,
}

impl<'a> PipelineBuilder<'a, (), (), ()> {
    pub fn new(context: &'a Arc<Context>) -> Self {
        PipelineBuilder {
            shader: (),
            layout: (),
            workgroup: (),
            context: context,
        }
    }
}

impl<'a, T> PipelineBuilder<'a, String, PipelineLayout<T>, WorkgroupSize> where T: Sized + std::fmt::Debug {
    pub fn build(self) -> Result<Arc<Pipeline<T>>> {
        Pipeline::new(self.context, self.layout, self.shader, self.workgroup)
    }
}

impl<'a, LayoutType, WorkgroupType> PipelineBuilder<'a, (), LayoutType, WorkgroupType> {
    pub fn shader<T: Into<String>>(self, shader: T) -> PipelineBuilder<'a, String, LayoutType, WorkgroupType> {
        PipelineBuilder {
            shader: shader.into(),
            layout: self.layout,
            workgroup: self.workgroup,
            context: self.context,
        }
    }
}

impl<'a, ShaderType, WorkgroupType> PipelineBuilder<'a, ShaderType, (), WorkgroupType> {
    pub fn layout<T>(self, count: usize) -> PipelineBuilder<'a, ShaderType, PipelineLayout<T>, WorkgroupType> where T: Sized + std::fmt::Debug {
        PipelineBuilder {
            shader: self.shader,
            layout: PipelineLayout::<T>::new(count),
            workgroup: self.workgroup,
            context: self.context,
        }
    }
}

impl<'a, ShaderType> PipelineBuilder<'a, ShaderType, (), ()> {
    pub fn layout_x<T>(self, count: usize) -> PipelineBuilder<'a, ShaderType, PipelineLayout<T>, WorkgroupSize> where T: Sized + std::fmt::Debug {
        PipelineBuilder {
            shader: self.shader,
            layout: PipelineLayout::<T>::new(count),
            workgroup: WorkgroupSize { x: count as u32, y: 1, z: 1 },
            context: self.context,
        }
    }
}

impl<'a, ShaderType, LayoutType> PipelineBuilder<'a, ShaderType, LayoutType, ()> {
    pub fn workgroup(self, x: u32, y: u32, z: u32) -> PipelineBuilder<'a, ShaderType, LayoutType, WorkgroupSize> {
        PipelineBuilder {
            shader: self.shader,
            layout: self.layout,
            workgroup: WorkgroupSize { x, y, z },
            context: self.context,
        }
    }
}

pub struct Pipeline<T> where T: Sized + std::fmt::Debug {
    layout: PipelineLayout<T>,
    context: Arc<Context>,
    device: Arc<Device>,
    command_pool: Arc<CommandPool>,
    shader_module: Arc<ShaderModule>,
    staging_buffer: Arc<StagingBuffer>,
    compute_pipeline: Arc<ComputePipeline>,
    command_dispatch: Arc<CommandDispatch>,
    buffer_memory_layout: BufferMemoryLayout<T>,
}

impl<T> Pipeline<T> where T: Sized + std::fmt::Debug {
    fn new<S: Into<String>>(
        context: &Arc<Context>, 
        layout: PipelineLayout<T>, 
        shader: S, 
        workgroup_size: WorkgroupSize) -> Result<Arc<Self>> {

        let instance = context.instance();
        let device = DeviceBuilder::new(&instance).build()?;
        println!("GPU: {:?}", device.physical_device().properties().device_name());
        let command_pool = CommandPool::new(&device)?;
        let shader_module = ShaderModule::new(&device, shader).unwrap();
        let buffer_memory_layout = BufferMemoryLayout::<T>::new(layout.count);
        let staging_buffer = StagingBuffer::new(&command_pool, &buffer_memory_layout);
        let compute_pipeline = ComputePipeline::new(&staging_buffer, &shader_module);
        let command_dispatch = CommandDispatch::new(&compute_pipeline, workgroup_size);
        let pipeline = Pipeline {
            layout: layout,
            context: Arc::clone(context),
            device: device,
            command_pool: command_pool,
            shader_module: shader_module,
            staging_buffer: staging_buffer,
            compute_pipeline: compute_pipeline,
            command_dispatch: command_dispatch,
            buffer_memory_layout: buffer_memory_layout,
        };
        Ok(Arc::new(pipeline))
    }

    pub fn compute(&self, input: &mut [T]) {
        self.staging_buffer.write_host_memory(&self.buffer_memory_layout, input);
        self.command_dispatch.dispatch();
        self.staging_buffer.read_host_memory(&self.buffer_memory_layout, input);
        println!("{:?}", input);
    }
}
