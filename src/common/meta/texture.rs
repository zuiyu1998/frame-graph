use std::sync::Arc;

use crate::{
    BindGroupTextureViewHandle, BindGroupTextureViewHandleHelper, FrameGraph, Handle,
    ResourceMaterial, TextureInfo, TextureViewInfo, TransientTexture,
};

#[derive(Clone, Debug)]
pub struct Texture {
    pub key: String,
    pub value: wgpu::Texture,
    pub desc: TextureInfo,
    pub default_texture_view_info: TextureViewInfo,
}

impl ResourceMaterial for Texture {
    type ResourceType = TransientTexture;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<TransientTexture> {
        let texture = Arc::new(TransientTexture {
            resource: self.value.clone(),
            desc: self.desc.clone(),
        });
        frame_graph.import(&self.key, texture)
    }
}

impl BindGroupTextureViewHandleHelper for Texture {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle {
        let texture = self.imported(frame_graph);

        BindGroupTextureViewHandle {
            texture,
            texture_view_info: self.default_texture_view_info.clone(),
        }
    }
}

pub struct TextureMeta {
    pub key: String,
    pub desc: TextureInfo,
    pub texture_view_info: TextureViewInfo,
}

impl ResourceMaterial for TextureMeta {
    type ResourceType = TransientTexture;

    fn imported(&self, frame_graph: &mut FrameGraph) -> Handle<TransientTexture> {
        frame_graph.get_or_create(&self.key, self.desc.clone())
    }
}

impl BindGroupTextureViewHandleHelper for TextureMeta {
    fn make_bind_group_texture_view_handle(
        &self,
        frame_graph: &mut FrameGraph,
    ) -> BindGroupTextureViewHandle {
        let texture = self.imported(frame_graph);

        BindGroupTextureViewHandle {
            texture,
            texture_view_info: self.texture_view_info.clone(),
        }
    }
}
