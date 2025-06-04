use std::sync::Arc;

use crate::{
    BindGroupBufferHandle, BindGroupBufferHandleHelper, BufferInfo, FrameGraph, Handle,
    ResourceMaterial, TransientBuffer, define_atomic_id,
};

define_atomic_id!(BufferId);

#[derive(Clone, Debug)]
pub struct Buffer {
    pub id: BufferId,
    pub value: wgpu::Buffer,
    pub desc: BufferInfo,
}

impl BindGroupBufferHandleHelper for Buffer {
    fn make_bind_group_buffer_handle(&self, frame_graph: &mut FrameGraph) -> BindGroupBufferHandle {
        let buffer = self.imported(frame_graph);

        BindGroupBufferHandle {
            buffer,
            size: None,
            offset: 0,
        }
    }
}

impl ResourceMaterial for Buffer {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<TransientBuffer> {
        let key = format!("buffer_{:?}", self.id);
        let buffer = Arc::new(TransientBuffer {
            resource: self.value.clone(),
            desc: self.desc.clone(),
        });
        frame_graph.import(&key, buffer)
    }
}

pub struct BufferMeta {
    pub key: String,
    pub desc: BufferInfo,
}

impl ResourceMaterial for BufferMeta {
    type ResourceType = TransientBuffer;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<TransientBuffer> {
        frame_graph.get_or_create(&self.key, self.desc.clone())
    }
}

impl BindGroupBufferHandleHelper for BufferMeta {
    fn make_bind_group_buffer_handle(&self, frame_graph: &mut FrameGraph) -> BindGroupBufferHandle {
        let buffer = self.imported(frame_graph);

        BindGroupBufferHandle {
            buffer,
            size: None,
            offset: 0,
        }
    }
}
