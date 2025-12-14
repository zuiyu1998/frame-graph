use crate::gfx_base::GpuPipelineLayout;

#[derive(Clone, Debug)]
pub struct GpuRenderPipeline(wgpu::RenderPipeline);

impl GpuRenderPipeline {
    pub fn wgpu(&self) -> &wgpu::RenderPipeline {
        &self.0
    }

    pub fn new(pipeline: wgpu::RenderPipeline) -> Self {
        GpuRenderPipeline(pipeline)
    }
}

pub struct RenderPipelineDescriptor {
    pub label: Option<String>,
    pub layout: GpuPipelineLayout,
}
