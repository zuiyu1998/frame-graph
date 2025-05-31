pub mod color_attachment;
pub mod texture_view;
pub mod depth_stencil_attachment;

pub use color_attachment::*;
pub use texture_view::*;
pub use depth_stencil_attachment::*;

use crate::RenderContext;

pub trait ResourceBinding {
    type Resource;

    fn make_resource<'a>(&self, render_context: &RenderContext<'a>) -> Self::Resource;
}