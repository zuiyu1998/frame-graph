pub mod bind_group_layout;
pub mod pipeline;
pub mod pipeline_cache;

use std::sync::Arc;

pub use bind_group_layout::*;
pub use pipeline::*;
pub use pipeline_cache::*;

use tracing::info;
use wgpu::{BindGroupLayoutEntry, Instance, RequestAdapterOptions};

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

#[derive(Clone)]
pub struct RenderQueue(pub Arc<wgpu::Queue>);

#[derive(Clone)]
pub struct RenderAdapter(pub Arc<wgpu::Adapter>);

#[derive(Clone)]
pub struct RenderInstance(pub Arc<wgpu::Instance>);

#[derive(Clone)]
pub struct RenderAdapterInfo(pub Arc<wgpu::AdapterInfo>);

pub async fn initialize_resources(
    instance: Instance,
    request_adapter_options: &RequestAdapterOptions<'_, '_>,
) -> (
    RenderDevice,
    RenderQueue,
    RenderAdapter,
    RenderAdapterInfo,
    RenderInstance,
) {
    let adapter = instance
        .request_adapter(request_adapter_options)
        .await
        .expect("Unable to find a GPU! Make sure you have installed required drivers!");

    let adapter_info = adapter.get_info();
    info!("{:?}", adapter_info);

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    (
        RenderDevice { device },
        RenderQueue(Arc::new(queue)),
        RenderAdapter(Arc::new(adapter)),
        RenderAdapterInfo(Arc::new(adapter_info)),
        RenderInstance(Arc::new(instance)),
    )
}
