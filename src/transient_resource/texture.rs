use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcTransientResource,
    IntoArcTransientResource, TransientResource, TransientResourceDescriptor,
};
use std::borrow::Cow;
use std::sync::Arc;

impl IntoArcTransientResource for TransientTexture {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource {
        ArcTransientResource::Texture(self)
    }
}

pub struct TransientTexture {
    pub resource: wgpu::Texture,
    pub desc: TextureInfo,
}

impl TransientResource for TransientTexture {
    type Descriptor = TextureInfo;

    fn borrow_resource(res: &AnyTransientResource) -> &Self {
        match res {
            AnyTransientResource::OwnedTexture(res) => res,
            AnyTransientResource::ImportedTexture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }

    fn get_desc(&self) -> &Self::Descriptor {
        &self.desc
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TextureInfo {
    pub label: Option<Cow<'static, str>>,
    pub size: wgpu::Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: wgpu::TextureDimension,
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsages,
    pub view_formats: Vec<wgpu::TextureFormat>,
}

impl From<TextureInfo> for AnyTransientResourceDescriptor {
    fn from(value: TextureInfo) -> Self {
        AnyTransientResourceDescriptor::Texture(value)
    }
}

impl TransientResourceDescriptor for TextureInfo {
    type Resource = TransientTexture;

    fn borrow_resource_descriptor(res: &AnyTransientResourceDescriptor) -> &Self {
        match res {
            AnyTransientResourceDescriptor::Texture(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}
