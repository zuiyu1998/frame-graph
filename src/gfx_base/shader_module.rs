use wgpu::ShaderModule as WgpuShaderModule;

pub struct GpuShaderModule(WgpuShaderModule);

impl GpuShaderModule {
    pub fn new(value: WgpuShaderModule) -> Self {
        GpuShaderModule(value)
    }

    pub(crate) fn get_wgpu_shader_module(&self) -> &WgpuShaderModule {
        &self.0
    }
}
