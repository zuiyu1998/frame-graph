use std::borrow::Cow;

use crate::RenderContext;

use super::{
    ColorAttachment, ColorAttachmentOwned, DepthStencilAttachment, DepthStencilAttachmentOwned,
    ResourceBinding,
};

#[derive(Default)]
pub struct RenderPassInfo {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachment>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachment>,
    pub raw_color_attachments: Vec<Option<ColorAttachmentOwned>>,
}

pub struct RenderPassInfoOwned {
    pub label: Option<Cow<'static, str>>,
    pub color_attachments: Vec<Option<ColorAttachmentOwned>>,
    pub depth_stencil_attachment: Option<DepthStencilAttachmentOwned>,
}

impl ResourceBinding for RenderPassInfo {
    type Resource = RenderPassInfoOwned;

    fn make_resource(&self, render_context: &RenderContext<'_>) -> Self::Resource {
        let mut color_attachments = self.raw_color_attachments.clone();

        for color_attachment in self.color_attachments.iter() {
            if color_attachment.is_none() {
                color_attachments.push(None);
            } else {
                color_attachments.push(Some(
                    color_attachment
                        .as_ref()
                        .unwrap()
                        .make_resource(render_context),
                ));
            }
        }

        let mut depth_stencil_attachment = None;

        if let Some(depth_stencil_attachment_blue_print) = &self.depth_stencil_attachment {
            depth_stencil_attachment =
                Some(depth_stencil_attachment_blue_print.make_resource(render_context));
        }

        RenderPassInfoOwned {
            label: self.label.clone(),
            color_attachments,
            depth_stencil_attachment,
        }
    }
}

impl RenderPassInfoOwned {
    pub fn create_render_pass<'a>(
        &self,
        command_encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        let depth_stencil_attachment =
            self.depth_stencil_attachment
                .as_ref()
                .map(|depth_stencil_attachment| {
                    depth_stencil_attachment.get_render_pass_depth_stencil_attachment()
                });

        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label.as_deref(),
            color_attachments: &self
                .color_attachments
                .iter()
                .map(|color_attachment| {
                    color_attachment
                        .as_ref()
                        .map(|color_attachment| color_attachment.get_render_pass_color_attachment())
                })
                .collect::<Vec<_>>(),
            depth_stencil_attachment,
            ..Default::default()
        });

        render_pass
    }
}
