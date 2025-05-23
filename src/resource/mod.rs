#[derive(Clone)]
pub struct RenderDevice {
    device: wgpu::Device,
}

impl RenderDevice {
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
}
