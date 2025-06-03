pub mod bind_group_binding;
pub mod bind_group_entry_binding;
pub mod bind_group_entry_handle;
pub mod bind_group_handle_builder;
pub mod bind_group_layout_entries;

pub use bind_group_binding::*;
pub use bind_group_entry_binding::*;
pub use bind_group_entry_handle::*;
pub use bind_group_handle_builder::*;
pub use bind_group_layout_entries::*;

use crate::{FrameGraph, PassNodeBuilder};

pub trait BindGroupResourceBindingHelper {
    fn make_bind_group_resource_binding(
        &self,
        pass_node_builder: &mut PassNodeBuilder,
    ) -> BindGroupResourceBinding;
}

pub trait BindGroupResourceHandleHelper {
    fn make_bind_group_resource_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupResourceHandle;
}

pub trait BindGroupTextureViewHandleHelper {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle;
}

pub trait BindGroupBufferHandleHelper {
    fn make_bind_group_buffer_handle(&self, frame_graph: &mut FrameGraph) -> BindGroupBufferHandle;
}
