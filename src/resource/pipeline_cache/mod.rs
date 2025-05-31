use super::{ComputePipeline, RenderPipeline};

pub type CachedPipelineId = usize;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedRenderPipelineId(CachedPipelineId);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedComputePipelineId(CachedPipelineId);

pub trait PipelineCacheTrait {
    fn get_render_pipeline(&self, id: CachedRenderPipelineId) -> Option<&RenderPipeline>;

    fn get_compute_pipeline(&self, id: CachedComputePipelineId) -> Option<&ComputePipeline>;
}
