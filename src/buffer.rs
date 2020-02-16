
use super::context::{Context};
use super::device::{Device, DeviceBuilder, CommandPool, ShaderModule};
use super::dispatch::{StagingBuffer, ComputePipeline, CommandDispatch};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;

pub struct Buffer {
    context: Arc<Context>,
    staging_buffer: Arc<StagingBuffer>,
}

impl Buffer {
    pub fn new(context: &Arc<Context>, region_sizes: Vec<usize>) -> Arc<Self> {
        let command_pool = context.command_pool();
        let staging_buffer = StagingBuffer::new(&command_pool, region_sizes.as_slice());
        let buffer = Buffer {
            context: Arc::clone(context),
            staging_buffer: staging_buffer,
        };
        Arc::new(buffer)
    }

    #[inline]
    pub(crate) fn context(&self) -> &Arc<Context> {
        &self.context
    }

    #[inline]
    pub(crate) fn staging_buffer(&self) -> &Arc<StagingBuffer> {
        &self.staging_buffer
    }
}
