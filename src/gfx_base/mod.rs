mod bind_group;
mod bind_group_layout;
mod buffer;
mod command_encoder;
mod pipeline;
mod render_device;
mod render_pass;
mod resource_macros;
mod sampler;
mod surface;
mod texture;
mod texture_view;

pub use bind_group::*;
pub use bind_group_layout::*;
pub use buffer::*;
pub use command_encoder::*;
pub use pipeline::*;
pub use render_device::*;
pub use render_pass::*;
pub use sampler::*;
pub use surface::*;
pub use texture::*;
pub use texture_view::*;

use wgpu::{Instance, SurfaceTargetUnsafe};

use std::sync::Arc;

pub struct RenderInstance(pub Arc<Instance>);

impl RenderInstance {
    // SAFETY: The window handles in ExtractedWindows will always be valid objects to create surfaces on
    pub fn create_surface_unsafe(&self, target: SurfaceTargetUnsafe) -> GpuSurface {
        let surface = unsafe {
            self.0
                .create_surface_unsafe(target)
                .expect("Failed to create wgpu surface")
        };

        GpuSurface::new(surface)
    }
}
