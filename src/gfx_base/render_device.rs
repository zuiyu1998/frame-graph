use wgpu::{CommandEncoder, Device};

use super::{BufferDescriptor, CommandEncoderDescriptor, GpuBuffer, GpuTexture, TextureDescriptor};

#[derive(Debug, Clone)]
pub struct RenderDevice {
    device: Device,
}

impl RenderDevice {
    pub fn create_command_encoder(&self, desc: &CommandEncoderDescriptor) -> CommandEncoder {
        self.device.create_command_encoder(&desc.get_buffer_desc())
    }

    pub fn create_texture(&self, desc: &TextureDescriptor) -> GpuTexture {
        let texture = self.device.create_texture(&desc.get_wgpu_desc());
        GpuTexture::new(texture)
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> GpuBuffer {
        let buffer = self.device.create_buffer(&desc.get_wgpu_desc());
        GpuBuffer::new(buffer)
    }
}
