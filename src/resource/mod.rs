pub mod pipeline;
pub mod pipeline_cache;

pub use pipeline::*;
pub use pipeline_cache::*;

#[derive(Clone)]
pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
}
