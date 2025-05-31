pub mod pipeline;
pub mod pipeline_cache;

pub use pipeline::*;
pub use pipeline_cache::*;

#[derive(Clone)]
pub struct Sampler {
    sampler: wgpu::Sampler,
}

impl Sampler {
    pub fn wgpu_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

#[derive(Clone)]
pub struct BindGroupLayout {
    layout: wgpu::BindGroupLayout,
}

impl BindGroupLayout {
    pub fn wgpu_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

#[derive(Clone)]
pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
}
