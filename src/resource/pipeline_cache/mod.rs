use super::{ComputePipeline, Pipeline, RenderPipeline};

pub type CachedPipelineId = usize;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedRenderPipelineId(CachedPipelineId);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedComputePipelineId(CachedPipelineId);

pub trait PipelineStorage {
    fn get_pipeline_cache(&self) -> PipelineCache;
}

pub struct PipelineCache(Vec<Option<Pipeline>>);

impl PipelineCache {
    pub fn new(value: Vec<Option<Pipeline>>) -> Self {
        Self(value)
    }

    pub fn get_render_pipeline(&self, id: CachedRenderPipelineId) -> Option<&RenderPipeline> {
        self.0[id.0]
            .as_ref()
            .and_then(|pipelie| pipelie.get_render_pipeline())
    }

    pub fn get_compute_pipeline(&self, id: CachedComputePipelineId) -> Option<&ComputePipeline> {
        self.0[id.0]
            .as_ref()
            .and_then(|pipelie| pipelie.get_compute_pipeline())
    }
}
