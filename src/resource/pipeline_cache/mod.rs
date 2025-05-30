use super::{ComputePipeline, RenderPipeline};

pub type CachedPipelineId = usize;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedRenderPipelineId(CachedPipelineId);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CachedComputePipelineId(CachedPipelineId);

pub trait PipelineCacheTrait {
    fn get_render_pipiline(&self, id: CachedRenderPipelineId) -> Option<&RenderPipeline>;

    fn get_compute_pipiline(&self, id: CachedRenderPipelineId) -> Option<&ComputePipeline>;
}
