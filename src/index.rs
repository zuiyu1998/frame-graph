use core::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use std::any::{Any, TypeId};

pub struct TypeIndex<T> {
    pub index: usize,
    _marker: PhantomData<T>,
}

impl<T: Any> Hash for TypeIndex<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        let id = TypeId::of::<T>();
        id.hash(state);
    }
}

impl<T> Eq for TypeIndex<T> {}

impl<T> PartialEq for TypeIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Copy for TypeIndex<T> {}

impl<T> Clone for TypeIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> TypeIndex<T> {
    pub fn new(index: usize) -> Self {
        TypeIndex {
            index,
            _marker: PhantomData,
        }
    }
}
