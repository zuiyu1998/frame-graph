use crate::PipelineCacheTrait;

pub struct RenderContext<'a, PipelineCache: PipelineCacheTrait> {
    pipeline_cache: &'a PipelineCache,
}
