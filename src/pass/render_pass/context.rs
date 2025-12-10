use crate::{PassContext, Ref, ResourceRead, TransientBuffer, gfx_base::GpuRenderPass};

pub struct RenderPassContext<'a, 'b> {
    render_pass: GpuRenderPass,
    pass_context: &'b mut PassContext<'a>,
}

impl<'a, 'b> RenderPassContext<'a, 'b> {
    pub fn new(render_pass: GpuRenderPass, pass_context: &'b mut PassContext<'a>) -> Self {
        RenderPassContext {
            render_pass,
            pass_context,
        }
    }

    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);
        self.render_pass.get_render_pass_mut().set_vertex_buffer(
            slot,
            buffer
                .resource
                .get_wgpu_buffer()
                .slice(offset..(offset + size)),
        );
    }

    pub fn set_index_buffer(
        &mut self,
        buffer_ref: &Ref<TransientBuffer, ResourceRead>,
        index_format: wgpu::IndexFormat,
        offset: u64,
        size: u64,
    ) {
        let buffer = self.pass_context.resource_table.get_resource(buffer_ref);

        self.render_pass.get_render_pass_mut().set_index_buffer(
            buffer
                .resource
                .get_wgpu_buffer()
                .slice(offset..(offset + size)),
            index_format,
        );
    }
}
