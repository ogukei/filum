
use super::instance::{Instance};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

use super::error::Result;
use std::sync::Arc;

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
    value: T,
}

impl<T> PipelineLayout<T> where T: Sized + std::fmt::Debug {
    fn new(count: usize) -> Self {
        PipelineLayout {
            count: count,
            value: unsafe { std::mem::zeroed() },
        }
    }
}

pub struct PipelineBuilder<'a, ShaderType, LayoutType> {
    shader: ShaderType,
    layout: LayoutType,
    context: &'a Arc<Context>,
}

impl<'a> PipelineBuilder<'a, (), ()> {
    pub fn new(context: &'a Arc<Context>) -> Self {
        PipelineBuilder {
            shader: (),
            layout: (),
            context: context,
        }
    }
}

impl<'a, T> PipelineBuilder<'a, String, PipelineLayout<T>> where T: Sized + std::fmt::Debug {
    pub fn build(self) -> Result<Arc<Pipeline<T>>> {
        Pipeline::new(self.context, self.layout, self.shader)
    }
}

impl<'a, LayoutType> PipelineBuilder<'a, (), LayoutType> {
    pub fn shader<T: Into<String>>(self, shader: T) -> PipelineBuilder<'a, String, LayoutType> {
        PipelineBuilder {
            shader: shader.into(),
            layout: self.layout,
            context: self.context,
        }
    }
}

impl<'a, ShaderType> PipelineBuilder<'a, ShaderType, ()> {
    pub fn layout<T>(self, count: usize) -> PipelineBuilder<'a, ShaderType, PipelineLayout<T>> where T: Sized + std::fmt::Debug {
        PipelineBuilder {
            shader: self.shader,
            layout: PipelineLayout::<T>::new(count),
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
}

impl<T> Pipeline<T> where T: Sized + std::fmt::Debug {
    fn new<S: Into<String>>(context: &Arc<Context>, layout: PipelineLayout<T>, shader: S) -> Result<Arc<Self>> {
        let instance = context.instance();
        let device = DeviceBuilder::new(&instance).build()?;
        let command_pool = CommandPool::new(&device)?;
        let shader_module = ShaderModule::new(&device, shader).unwrap();
        let staging_buffer = StagingBuffer::new(&command_pool);
        let compute_pipeline = ComputePipeline::new(&staging_buffer, &shader_module);
        let pipeline = Pipeline {
            layout: layout,
            context: Arc::clone(context),
            device: device,
            command_pool: command_pool,
            shader_module: shader_module,
            staging_buffer: staging_buffer,
            compute_pipeline: compute_pipeline,
        };
        Ok(Arc::new(pipeline))
    }

    pub fn compute(&self, input: &Vec<T>) {
        let mut vec = Vec::<T>::with_capacity(self.layout.count);
        vec.resize_with(self.layout.count, || unsafe { std::mem::zeroed() });
        println!("{:?}", vec);
        let command_dispatch = CommandDispatch::new(&self.compute_pipeline);
        println!("{:?}", command_dispatch.output);
    }
}
