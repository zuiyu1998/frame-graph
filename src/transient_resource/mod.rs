mod buffer;
mod cache;
mod texture;

pub use buffer::*;
pub use cache::*;
pub use texture::*;

use crate::RenderDevice;
use std::sync::Arc;

pub trait TransientResourceCreator {
    fn create_resource(&self, desc: &AnyTransientResourceDescriptor) -> AnyTransientResource;
}

impl TransientResourceCreator for RenderDevice {
    fn create_resource(&self, _desc: &AnyTransientResourceDescriptor) -> AnyTransientResource {
        todo!()
    }
}

#[derive(Clone)]
pub enum ArcTransientResource {
    Buffer(Arc<TransientBuffer>),
    Texture(Arc<TransientTexture>),
}

pub trait IntoArcTransientResource {
    fn into_arc_transient_resource(self: Arc<Self>) -> ArcTransientResource;
}

pub enum AnyTransientResource {
    OwnedBuffer(TransientBuffer),
    ImportedBuffer(Arc<TransientBuffer>),
    OwnedTexture(TransientTexture),
    ImportedTexture(Arc<TransientTexture>),
}

impl From<TransientBuffer> for AnyTransientResource {
    fn from(value: TransientBuffer) -> Self {
        AnyTransientResource::OwnedBuffer(value)
    }
}

impl From<Arc<TransientBuffer>> for AnyTransientResource {
    fn from(value: Arc<TransientBuffer>) -> Self {
        AnyTransientResource::ImportedBuffer(value)
    }
}

impl From<TransientTexture> for AnyTransientResource {
    fn from(value: TransientTexture) -> Self {
        AnyTransientResource::OwnedTexture(value)
    }
}

impl From<Arc<TransientTexture>> for AnyTransientResource {
    fn from(value: Arc<TransientTexture>) -> Self {
        AnyTransientResource::ImportedTexture(value)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum AnyTransientResourceDescriptor {
    Buffer(BufferInfo),
    Texture(TextureInfo),
}

pub trait TransientResource: 'static {
    type Descriptor: TransientResourceDescriptor;

    fn borrow_resource(res: &AnyTransientResource) -> &Self;

    fn get_desc(&self) -> &Self::Descriptor;
}

pub trait TransientResourceDescriptor:
    'static + Clone + Into<AnyTransientResourceDescriptor>
{
    type Resource: TransientResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}
