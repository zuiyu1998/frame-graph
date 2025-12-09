use crate::{
    frame_graph::pass::{PassCommand, PassContext},
    gfx_base::{RenderPassColorAttachment, RenderPassDescriptor},
};

pub struct RenderPassColorAttachmentDescriptor {}

pub struct RenderPassDepthStencilAttachmentDescriptor {}

pub struct TransientRenderPassDescriptor {
    label: Option<String>,
    color_attachments: Option<RenderPassColorAttachmentDescriptor>,
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDescriptor>,
}

#[derive(Default)]
pub struct RenderPass {}

impl PassCommand for RenderPass {
    fn execute(&self, context: &mut PassContext) {}
}
