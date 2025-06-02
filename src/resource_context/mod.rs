pub mod compute_pass_context;
pub mod encoder;
pub mod encoder_pass_context;
pub mod parameter;
pub mod render_pass_context;

pub use compute_pass_context::*;
pub use encoder::*;
pub use encoder_pass_context::*;
pub use parameter::*;
pub use render_pass_context::*;

use wgpu::CommandEncoder;

use crate::{
    CachedComputePipelineId, CachedRenderPipelineId, ComputePipeline, PipelineCache,
    PipelineStorage, Ref, RenderDevice, RenderPassOwned, RenderPipeline, ResourceTable,
    ResourceView, TransientResource, TransientResourceCache,
};

pub struct RenderContext<'a> {
    pub render_device: &'a RenderDevice,
    pub transient_resource_cache: &'a mut TransientResourceCache,
    pub(crate) resource_table: ResourceTable,
    command_buffer_queue: Vec<wgpu::CommandBuffer>,
    pipeline_cache: PipelineCache,
}

impl<'a> RenderContext<'a> {
    pub fn new<T: PipelineStorage>(
        render_device: &'a RenderDevice,
        transient_resource_cache: &'a mut TransientResourceCache,
        pipeline_storage: &'a T,
    ) -> Self {
        Self {
            render_device,
            transient_resource_cache,
            command_buffer_queue: vec![],
            pipeline_cache: pipeline_storage.get_pipeline_cache(),
            resource_table: ResourceTable::default(),
        }
    }

    pub fn begin_render_pass<'b>(
        &'b mut self,
        command_encoder: &'b mut CommandEncoder,
        render_pass_owned: &RenderPassOwned,
    ) -> RenderPassContext<'a, 'b> {
        let render_pass = render_pass_owned.create_render_pass(command_encoder);

        RenderPassContext::new(command_encoder, render_pass, self)
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

    pub fn add_command_buffer(&mut self, command_buffer: wgpu::CommandBuffer) {
        self.command_buffer_queue.push(command_buffer);
    }

    pub fn finish(self) -> Vec<wgpu::CommandBuffer> {
        self.command_buffer_queue
    }
}
