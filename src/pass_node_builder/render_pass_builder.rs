use std::mem::take;

use crate::{Ref, RenderPass, ResourceRead, TransientBuffer};

use super::{PassBuilder, RenderPassExt};

pub struct RenderPassBuilder<'a, 'b> {
    render_pass: RenderPass,
    pass_builder: &'b mut PassBuilder<'a>,
}

impl<'a, 'b> RenderPassBuilder<'a, 'b> {
    pub fn new(pass_builder: &'b mut PassBuilder<'a>, name: &str) -> Self {
        let mut render_pass = RenderPass::default();
        render_pass.set_pass_name(name);

        Self {
            render_pass,
            pass_builder,
        }
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) -> &mut Self {
        self.render_pass
            .set_index_buffer(buffer_ref, index_format, offset, size);

        self
    }

    pub fn create_render_pass_builder(&mut self) -> &mut Self {
        self.finish();

        self
    }

    fn finish(&mut self) {
        let render_pass = take(&mut self.render_pass);
        self.pass_builder.push(render_pass);
    }
}
