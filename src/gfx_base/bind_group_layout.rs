use wgpu::{BindGroupLayout as WgpuBindGroupLayout, BindGroupLayoutEntry};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[derive(Clone, Debug)]
pub struct GpuBindGroupLayout(WgpuBindGroupLayout);
