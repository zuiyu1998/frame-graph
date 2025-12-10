use crate::RenderPass;

use super::PassBuilder;

pub struct RenderPassBuilder<'a, 'b> {
    render_pass: RenderPass,
    pass_builder: &'b mut PassBuilder<'a>,
}

impl<'a, 'b> RenderPassBuilder<'a, 'b> {
    pub fn new(pass_builder: &'b mut PassBuilder<'a>, name: &str) -> Self {
        let mut render_pass = RenderPass::default();
        render_pass.set_pass_name(name);

        Self {
            render_pass,
            pass_builder,
        }
    }
}
