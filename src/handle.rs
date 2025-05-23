use core::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use std::any::{Any, TypeId};

pub struct TypeHandle<T> {
    pub index: usize,
    _marker: PhantomData<T>,
}

impl<T: Any> Hash for TypeHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        let id = TypeId::of::<T>();
        id.hash(state);
    }
}

impl<T> Eq for TypeHandle<T> {}

impl<T> PartialEq for TypeHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Copy for TypeHandle<T> {}

impl<T> Clone for TypeHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> TypeHandle<T> {
    pub fn new(index: usize) -> Self {
        TypeHandle {
            index,
            _marker: PhantomData,
        }
    }
}
