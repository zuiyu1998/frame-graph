use crate::{
    CachedComputePipelineId, CachedRenderPipelineId, ComputePipeline, PipelineCacheTrait, Ref,
    RenderDevice, RenderPipeline, ResourceTable, ResourceView, TransientResource,
    TransientResourceCache,
};

pub struct RenderContext<'a, PipelineCache> {
    pub(crate) render_device: &'a RenderDevice,
    pub(crate) transient_resource_cache: &'a mut TransientResourceCache,
    pub(crate) resource_table: ResourceTable,
    command_buffer_queue: Vec<wgpu::CommandBuffer>,
    pipeline_cache: &'a PipelineCache,
}

impl<'a, PipelineCache: PipelineCacheTrait> RenderContext<'a, PipelineCache> {
    pub fn new(
        render_device: &'a RenderDevice,
        transient_resource_cache: &'a mut TransientResourceCache,
        pipeline_cache: &'a PipelineCache,
    ) -> Self {
        Self {
            render_device,
            transient_resource_cache,
            command_buffer_queue: vec![],
            pipeline_cache,
            resource_table: ResourceTable::default(),
        }
    }

    pub fn get_resource<ResourceType: TransientResource, View: ResourceView>(
        &self,
        resource_ref: &Ref<ResourceType, View>,
    ) -> &ResourceType {
        self.resource_table
            .get_resource(resource_ref)
            .expect("resource mut have")
    }

    pub fn get_compute_pipeline(&self, id: CachedComputePipelineId) -> &ComputePipeline {
        self.pipeline_cache
            .get_compute_pipeline(id)
            .expect("compute pipeline mut have")
    }

    pub fn get_render_pipeline(&self, id: CachedRenderPipelineId) -> &RenderPipeline {
        self.pipeline_cache
            .get_render_pipeline(id)
            .expect("render pipeline mut have")
    }

    pub fn device(&self) -> &RenderDevice {
        self.render_device
    }

    pub fn add_command_buffer(&mut self, command_buffer: wgpu::CommandBuffer) {
        self.command_buffer_queue.push(command_buffer);
    }

    pub fn finish(self) -> Vec<wgpu::CommandBuffer> {
        self.command_buffer_queue
    }
}
