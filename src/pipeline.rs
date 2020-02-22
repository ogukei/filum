

use super::context::{Context};
use super::buffer::{Buffer};

use super::instance::{Instance};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule, ShaderModuleSource};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch, WorkgroupCount, ConstantEntry};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;

#[macro_export]
macro_rules! constants {
    ($($v:expr),*) => {
        {
            fn copy_value<T: Copy>(v: T) -> T { v }
            let mut entries = Vec::<$crate::ConstantEntry>::new();
            $(
                {
                    let v = copy_value($v);
                    let size = std::mem::size_of_val(&v);
                    let slice = unsafe {
                        std::slice::from_raw_parts(&v as *const _ as *const u8, size)
                    };
                    let entry = $crate::ConstantEntry::new(slice.to_vec());
                    entries.push(entry);
                }
            )*
            entries
        }
    };
}

pub struct PipelineBuilder<'a, ShaderType, SpecializationType> {
    shader: ShaderType,
    specialization: SpecializationType,
    buffer: &'a Arc<Buffer>,
}

impl<'a> PipelineBuilder<'a, (), ()> {
    pub fn new(buffer: &'a Arc<Buffer>) -> Self {
        PipelineBuilder {
            shader: (),
            specialization: (),
            buffer: buffer,
        }
    }
}

impl<'a, SpecializationType> PipelineBuilder<'a, (), SpecializationType> {
    pub fn shader(self, filename: impl Into<String>) -> PipelineBuilder<'a, ShaderModuleSource, SpecializationType> {
        PipelineBuilder {
            shader: ShaderModuleSource::from_file(filename),
            specialization: self.specialization,
            buffer: self.buffer,
        }
    }

    pub fn shader_bytes(self, bytes: Vec<u8>) -> PipelineBuilder<'a, ShaderModuleSource, SpecializationType> {
        PipelineBuilder {
            shader: ShaderModuleSource::from_bytes(bytes),
            specialization: self.specialization,
            buffer: self.buffer,
        }
    }
}

impl<'a, ShaderType> PipelineBuilder<'a, ShaderType, ()> {
    pub fn specialization(self, constants: Vec<ConstantEntry>) 
        -> PipelineBuilder<'a, ShaderType, Vec<ConstantEntry>> {
        PipelineBuilder {
            shader: self.shader,
            specialization: constants,
            buffer: self.buffer,
        }
    }
}

impl<'a> PipelineBuilder<'a, ShaderModuleSource, ()> {
    pub fn build(self) -> Result<Arc<Pipeline>> {
        Pipeline::new(self.buffer, self.shader, vec![])
    }
}

impl<'a> PipelineBuilder<'a, ShaderModuleSource, Vec<ConstantEntry>> {
    pub fn build(self) -> Result<Arc<Pipeline>> {
        Pipeline::new(self.buffer, self.shader, self.specialization)
    }
}

pub struct Pipeline {
    buffer: Arc<Buffer>,
    shader_module: Arc<ShaderModule>,
    compute_pipeline: Arc<ComputePipeline>,
}

impl Pipeline {
    fn new(buffer: &Arc<Buffer>, shader: ShaderModuleSource, spec_constants: Vec<ConstantEntry>) -> Result<Arc<Self>> {
        let context = buffer.context();
        let device = context.device();
        let staging_buffer = buffer.staging_buffer();
        let shader_module = ShaderModule::new(device, shader)?;
        let compute_pipeline = ComputePipeline::new(staging_buffer, &shader_module, spec_constants);
        let pipeline = Pipeline {
            buffer: Arc::clone(buffer),
            shader_module: shader_module,
            compute_pipeline: compute_pipeline,
        };
        Ok(Arc::new(pipeline))
    }

    pub fn dispatch(&self, count_x: usize) {
        let count = WorkgroupCount { x: count_x as u32, y: 1, z: 1 };
        let dispatch = CommandDispatch::new(&self.compute_pipeline, count, vec![]);
        dispatch.dispatch();
    }
}

pub struct DispatchBuilder<'a, WorkgroupCountType, PushConstantsType> {
    pipeline: &'a Arc<Pipeline>,
    count: WorkgroupCountType,
    push_constants: PushConstantsType,
}


impl<'a> DispatchBuilder<'a, (), ()> {
    pub fn new(pipeline: &'a Arc<Pipeline>) -> Self {
        DispatchBuilder {
            pipeline: pipeline,
            count: (),
            push_constants: (),
        }
    }
}

impl<'a, PushConstantsType> DispatchBuilder<'a, (), PushConstantsType> {
    pub fn workgroup_count(self, x: usize, y: usize, z: usize) -> DispatchBuilder<'a, WorkgroupCount, PushConstantsType> {
        DispatchBuilder {
            pipeline: self.pipeline,
            count: WorkgroupCount { x: x as u32, y: y as u32, z: z as u32 },
            push_constants: self.push_constants,
        }
    }
}

impl<'a, WorkgroupCountType> DispatchBuilder<'a, WorkgroupCountType, ()> {
    pub fn push_constants(self, push_constants: Vec<ConstantEntry>) 
        -> DispatchBuilder<'a, WorkgroupCountType, Vec<ConstantEntry>> {
        DispatchBuilder {
            pipeline: self.pipeline,
            count: self.count,
            push_constants: push_constants,
        }
    }
}

impl<'a> DispatchBuilder<'a, WorkgroupCount, Vec<ConstantEntry>> {
    pub fn build(self) -> Result<Arc<CommandDispatch>> {
        let compute_pipeline = &self.pipeline.compute_pipeline;
        Ok(CommandDispatch::new(compute_pipeline, self.count, self.push_constants))
    }
}

impl<'a> DispatchBuilder<'a, WorkgroupCount, ()> {
    pub fn build(self) -> Result<Arc<CommandDispatch>> {
        let compute_pipeline = &self.pipeline.compute_pipeline;
        Ok(CommandDispatch::new(compute_pipeline, self.count, vec![]))
    }
}
