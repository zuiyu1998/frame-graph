use wgpu::{
    BufferAddress, ColorTargetState, DepthStencilState, MultisampleState, PrimitiveState,
    VertexAttribute, VertexStepMode,
};

use crate::gfx_base::{GpuPipelineLayout, GpuShaderModule};

#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    pub array_stride: BufferAddress,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

impl VertexBufferLayout {
    pub fn get_wgpu_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: &self.attributes,
        }
    }
}

pub struct VertexState {
    pub module: GpuShaderModule,
    pub entry_point: Option<String>,
    pub buffers: Vec<VertexBufferLayout>,
}

pub struct FragmentState {
    pub module: GpuShaderModule,
    pub entry_point: Option<String>,
    pub targets: Vec<Option<ColorTargetState>>,
}

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
    pub vertex: VertexState,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub fragment: Option<FragmentState>,
}
