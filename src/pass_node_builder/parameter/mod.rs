mod set_index_buffer_parameter;

pub use set_index_buffer_parameter::*;

use crate::{Ref, RenderPass, RenderPassCommand, ResourceRead, TransientBuffer};

pub trait RenderPassExt {
    fn push<T: RenderPassCommand>(&mut self, value: T);

    fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) {
        self.push(SetIndexBufferParameter {
            buffer_ref: buffer_ref.clone(),
            index_format,
            offset,
            size,
        });
    }
}

impl RenderPassExt for RenderPass {
    fn push<T: RenderPassCommand>(&mut self, value: T) {
        self.commands.push(Box::new(value));
    }
}
