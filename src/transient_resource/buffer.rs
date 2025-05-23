use std::borrow::Cow;
use std::sync::Arc;
use wgpu::BufferAddress;

use super::{
    AnyTransientResource, AnyTransientResourceDescriptor, ArcTransientResource,
    IntoArcTransientResource, TransientResource, TransientResourceDescriptor,
};

impl IntoArcTransientResource for TransientBuffer {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource {
        ArcTransientResource::Buffer(self)
    }
}

pub struct TransientBuffer {
    pub resource: wgpu::Buffer,
    pub desc: BufferInfo,
}

impl TransientResource for TransientBuffer {
    type Descriptor = BufferInfo;

    fn borrow_resource(res: &AnyTransientResource) -> &Self {
        match res {
            AnyTransientResource::OwnedBuffer(res) => res,
            AnyTransientResource::ImportedBuffer(res) => res,
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
pub struct BufferInfo {
    pub label: Option<Cow<'static, str>>,
    pub size: BufferAddress,
    pub usage: wgpu::BufferUsages,
    pub mapped_at_creation: bool,
}

impl From<BufferInfo> for AnyTransientResourceDescriptor {
    fn from(value: BufferInfo) -> Self {
        AnyTransientResourceDescriptor::Buffer(value)
    }
}

impl TransientResourceDescriptor for BufferInfo {
    type Resource = TransientBuffer;
}
