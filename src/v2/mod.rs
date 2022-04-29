//! # Recommend V2 module
//!
//! In v2, no longer needs `Clone` trait
//!
//! And Permutate now allow two params
//!
//! What is more, it use Optional value and any input won't occur error
//!
//! Anyway, old api has not changed, but you are going to use v2 module, just add `v2` in the feature!
//!
//!
//! # Example
//! ```
//!use combination::v2::*;
//!let str_list = ["hi", "i", "am", "roger", "and", "you"];
//!let combine = Combine::new(6, 4);
//!let res = str_list.try_select(&combine).unwrap();
//!for v in res {
//!   println!("{:?}", v);
//!}
//!
//! ```
//!
//! As long as use two traits `Select` and `Selector` it will work.
//!
//! V2 module provides two structs which implement `Selector` trait, they are `Combine` and `Permutate`
//!
//! By using them with trait `Select`, type as `&[T], Vec<T>, [T]` will be able to be selected.
//!

mod internal;

mod all_dislocation;
mod combine;
mod permutate;
mod select;

pub use all_dislocation::AllDislocation;
pub use combine::Combine;
pub use permutate::Permutate;
pub use select::{InsufficientSize, Select, Selector};

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_select_from_str_list() {
        let list = ["it", "is", "a", "test", "example"];
        let p = Permutate::new(5, 3);
        let res = list.select(&p);

        for v in res {
            println!("{:?}", v);
        }
    }
}
