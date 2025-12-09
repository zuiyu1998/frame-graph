mod render_pass;

pub use render_pass::*;

use wgpu::{CommandBuffer, CommandEncoder};

use crate::gfx_base::{CommandEncoderDescriptor, RenderDevice};

pub struct PassContext<'a> {
    pub render_device: &'a RenderDevice,
    pub command_encoder: CommandEncoder,
}

impl PassContext<'_> {
    pub fn finish(self) -> CommandBuffer {
        self.command_encoder.finish()
    }
}

pub trait PassCommand: 'static + Send + Sync {
    fn execute(&self, context: &mut PassContext);
}

#[derive(Default)]
pub struct Pass {
    pub label: Option<String>,
    commands: Vec<Box<dyn PassCommand>>,
}

impl Pass {
    pub fn render(&self, command_buffers: &mut Vec<CommandBuffer>, render_device: &RenderDevice) {
        let command_encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
            label: self.label.clone(),
        });

        let mut pass_context = PassContext {
            render_device,
            command_encoder,
        };

        for command in self.commands.iter() {
            command.execute(&mut pass_context);
        }
        command_buffers.push(pass_context.finish());
    }
}
