use core::marker::PhantomData;

use crate::{AnyTransientResourceDescriptor, ArcTransientResource, PassNode, TypeHandle};

pub struct Ref<ResourceType, VieType> {
    pub handle: TypeHandle<ResourceNode>,
    _marker: PhantomData<(ResourceType, VieType)>,
}

impl<ResourceType, VieType> Ref<ResourceType, VieType> {
    pub fn new(handle: TypeHandle<ResourceNode>) -> Self {
        Self {
            handle,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType, VieType> Clone for Ref<ResourceType, VieType> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType, VieType> PartialEq for Ref<ResourceType, VieType> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl<ResourceType, VieType> Eq for Ref<ResourceType, VieType> {}

pub trait ResourceView {}

pub struct ResourceRead;
pub struct ResourceWrite;

impl ResourceView for ResourceRead {}
impl ResourceView for ResourceWrite {}

pub struct Handle<ResourceType> {
    pub raw: GraphRawResourceHandle,
    _marker: PhantomData<ResourceType>,
}

impl<ResourceType> Clone for Handle<ResourceType> {
    fn clone(&self) -> Self {
        Handle {
            raw: self.raw.clone(),
            _marker: PhantomData,
        }
    }
}

impl<ResourceType> Handle<ResourceType> {
    pub fn new(handle: TypeHandle<ResourceNode>, version: u32) -> Self {
        Self {
            raw: GraphRawResourceHandle { handle, version },
            _marker: PhantomData,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct GraphRawResourceHandle {
    pub handle: TypeHandle<ResourceNode>,
    pub version: u32,
}

pub struct ResourceNode {
    pub handle: TypeHandle<ResourceNode>,
    pub name: String,
    pub first_use_pass: Option<TypeHandle<PassNode>>,
    pub last_user_pass: Option<TypeHandle<PassNode>>,
    version: u32,
    pub resource: VirtualResource,
}

pub struct ResourceRequese {
    pub handle: TypeHandle<ResourceNode>,
    pub resource: VirtualResource,
}

pub struct ResourceRelease {
    pub handle: TypeHandle<ResourceNode>,
}

#[derive(Clone)]
pub enum VirtualResource {
    Setuped(AnyTransientResourceDescriptor),
    Imported(ArcTransientResource),
}

impl ResourceNode {
    pub fn new(name: &str, handle: TypeHandle<ResourceNode>, resource: VirtualResource) -> Self {
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

    pub fn update_lifetime(&mut self, handle: TypeHandle<PassNode>) {
        if self.first_use_pass.is_none() {
            self.first_use_pass = Some(handle);
        }

        self.last_user_pass = Some(handle);
    }
}
