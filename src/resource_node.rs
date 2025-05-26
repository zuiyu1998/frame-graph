use core::marker::PhantomData;

use crate::{
    AnyTransientResourceDescriptor, ArcTransientResource, PassNode, TransientResource, TypeIndex,
};

pub struct Ref<ResourceType: TransientResource, VieType> {
    pub raw: RawResourceHandle,
    pub desc: <ResourceType as TransientResource>::Descriptor,
    _marker: PhantomData<(ResourceType, VieType)>,
}

impl<ResourceType: TransientResource, VieType> Ref<ResourceType, VieType> {
    pub fn new(
        raw: RawResourceHandle,
        desc: <ResourceType as TransientResource>::Descriptor,
    ) -> Self {
        Self {
            raw,
            desc,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource, VieType> Clone for Ref<ResourceType, VieType> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            desc: self.desc.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource, VieType> PartialEq for Ref<ResourceType, VieType> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<ResourceType: TransientResource, VieType> Eq for Ref<ResourceType, VieType> {}

pub trait ResourceView {}

pub struct ResourceRead;
pub struct ResourceWrite;

impl ResourceView for ResourceRead {}

impl ResourceView for ResourceWrite {}

pub struct Handle<ResourceType: TransientResource> {
    pub raw: RawResourceHandle,
    pub desc: <ResourceType as TransientResource>::Descriptor,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType: TransientResource> Clone for Handle<ResourceType> {
    fn clone(&self) -> Self {
        Handle {
            raw: self.raw.clone(),
            desc: self.desc.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType: TransientResource> Handle<ResourceType> {
    pub fn new(
        handle: TypeIndex<ResourceNode>,
        version: u32,
        desc: <ResourceType as TransientResource>::Descriptor,
    ) -> Self {
        Self {
            raw: RawResourceHandle { handle, version },
            desc,
            _marker: PhantomData,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RawResourceHandle {
    pub handle: TypeIndex<ResourceNode>,
    pub version: u32,
}

pub struct ResourceNode {
    pub handle: TypeIndex<ResourceNode>,
    pub name: String,
    pub first_use_pass: Option<TypeIndex<PassNode>>,
    pub last_user_pass: Option<TypeIndex<PassNode>>,
    version: u32,
    pub resource: VirtualResource,
}

pub struct ResourceRequese {
    pub handle: TypeIndex<ResourceNode>,
    pub resource: VirtualResource,
}

pub struct ResourceRelease {
    pub handle: TypeIndex<ResourceNode>,
}

#[derive(Clone)]
pub enum VirtualResource {
    Setuped(AnyTransientResourceDescriptor),
    Imported(ArcTransientResource),
}

impl ResourceNode {
    pub fn new(name: &str, handle: TypeIndex<ResourceNode>, resource: VirtualResource) -> Self {
        ResourceNode {
            name: name.to_string(),
            handle,
            version: 0,
            first_use_pass: None,
            last_user_pass: None,
            resource,
        }
    }
}

impl ResourceNode {
    pub fn request(&self) -> ResourceRequese {
        ResourceRequese {
            handle: self.handle,
            resource: self.resource.clone(),
        }
    }

    pub fn release(&self) -> ResourceRelease {
        ResourceRelease {
            handle: self.handle,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1;
    }

    pub fn update_lifetime(&mut self, handle: TypeIndex<PassNode>) {
        if self.first_use_pass.is_none() {
            self.first_use_pass = Some(handle);
        }

        self.last_user_pass = Some(handle);
    }
}
