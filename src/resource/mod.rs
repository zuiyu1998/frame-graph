pub mod pipeline;
pub mod pipeline_cache;

pub use pipeline::*;
pub use pipeline_cache::*;
use wgpu::BindGroupLayoutEntry;

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

    #[inline]
    pub fn create_bind_group_layout<'a>(
        &self,
        label: impl Into<wgpu::Label<'a>>,
        entries: &'a [BindGroupLayoutEntry],
    ) -> BindGroupLayout {
        BindGroupLayout {
            layout: self
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: label.into(),
                    entries,
                }),
        }
    }
}
