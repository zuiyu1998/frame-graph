pub mod index;
pub mod resource;
pub mod resource_node;
pub mod resource_table;
pub mod transient_resource;

pub use index::*;
pub use resource::*;
pub use resource_node::*;
pub use resource_table::*;
pub use transient_resource::*;

pub struct PassNode {}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
