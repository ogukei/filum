

use super::context::{Context};
use super::buffer::{Buffer};

use super::instance::{Instance};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch, WorkgroupCount};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;

pub struct PipelineBuilder<'a, ShaderType> {
    shader: ShaderType,
    buffer: &'a Arc<Buffer>,
}

impl<'a> PipelineBuilder<'a, ()> {
    pub fn new(buffer: &'a Arc<Buffer>) -> Self {
        PipelineBuilder {
            shader: (),
            buffer: buffer,
        }
    }
}

impl<'a> PipelineBuilder<'a, ()> {
    pub fn shader(self, filename: impl Into<String>) -> PipelineBuilder<'a, String> {
        PipelineBuilder {
            shader: filename.into(),
            buffer: self.buffer,
        }
    }
}

impl<'a> PipelineBuilder<'a, String> {
    pub fn build(self) -> Result<Arc<Pipeline>> {
        Pipeline::new(self.buffer, self.shader)
    }
}

pub struct Pipeline {
    buffer: Arc<Buffer>,
    shader_module: Arc<ShaderModule>,
    compute_pipeline: Arc<ComputePipeline>,
}

impl Pipeline {
    fn new(buffer: &Arc<Buffer>, shader: String) -> Result<Arc<Self>> {
        let context = buffer.context();
        let device = context.device();
        let staging_buffer = buffer.staging_buffer();
        let shader_module = ShaderModule::new(device, shader).unwrap();
        let compute_pipeline = ComputePipeline::new(staging_buffer, &shader_module);
        let pipeline = Pipeline {
            buffer: Arc::clone(buffer),
            shader_module: shader_module,
            compute_pipeline: compute_pipeline,
        };
        Ok(Arc::new(pipeline))
    }
}


pub struct DispatchBuilder<'a, WorkgroupCountType> {
    pipeline: &'a Arc<Pipeline>,
    count: WorkgroupCountType,
}


impl<'a> DispatchBuilder<'a, ()> {
    pub fn new(pipeline: &'a Arc<Pipeline>) -> Self {
        DispatchBuilder {
            pipeline: pipeline,
            count: (),
        }
    }
}

impl<'a> DispatchBuilder<'a, ()> {
    pub fn workgroup_count(self, x: u32, y: u32, z: u32) -> DispatchBuilder<'a, WorkgroupCount> {
        DispatchBuilder {
            pipeline: self.pipeline,
            count: WorkgroupCount { x, y, z },
        }
    }
}

impl<'a> DispatchBuilder<'a, WorkgroupCount> {
    pub fn build(self) -> Result<Arc<CommandDispatch>> {
        let compute_pipeline = &self.pipeline.compute_pipeline;
        Ok(CommandDispatch::new(compute_pipeline, self.count))
    }
}
