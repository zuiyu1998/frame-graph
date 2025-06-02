use std::borrow::Cow;

use wgpu::CommandEncoder;

use crate::{EncoderCommand, RenderContext};

#[derive(Default)]
pub struct Pass {
    pub label: Option<Cow<'static, str>>,
    begin_encoder_commands: Vec<EncoderCommand>,
    executors: Vec<Box<dyn EncoderExecutor>>,
    end_encoder_commands: Vec<EncoderCommand>,
}

impl Pass {
    pub fn render(&self, render_context: &mut RenderContext) {
        let mut command_encoder = render_context
            .render_device
            .wgpu_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: self.label.as_deref(),
            });

        for begin_encoder_command in self.begin_encoder_commands.iter() {
            begin_encoder_command.apply(&mut command_encoder);
        }

        for executor in self.executors.iter() {
            executor.execute(&mut command_encoder, render_context);
        }

        for end_encoder_command in self.end_encoder_commands.iter() {
            end_encoder_command.apply(&mut command_encoder);
        }

        let command_buffer = command_encoder.finish();

        render_context.add_command_buffer(command_buffer);
    }
}

pub trait EncoderExecutor: 'static + Send + Sync {
    fn execute(&self, command_encoder: &mut CommandEncoder, render_context: &mut RenderContext);
}
