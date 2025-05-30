pub struct RenderPipeline(wgpu::RenderPipeline);

impl RenderPipeline {
    pub fn wgpu(&self) -> &wgpu::RenderPipeline {
        &self.0
    }
}

pub struct ComputePipeline(wgpu::ComputePipeline);

impl ComputePipeline {
    pub fn wgpu(&self) -> &wgpu::ComputePipeline {
        &self.0
    }
}
