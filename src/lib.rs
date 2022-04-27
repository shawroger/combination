//!  # What it is
//!  combination is a lib to do math jobs like permutate and combinate data from vec.
//!
//! ```rust
//! use combination::*;
//! fn test_permutation() {
//!     let val = permutate::from_vec(&vec![10, 20, 30, 40]);
//!     for item in val {
//!        println!("{:?}", item);
//!     }
//! }
//! ```
//!
//!
//!

pub mod combine;
pub mod permutate;

#[cfg(feature = "v2")]
pub mod v2;

pub use combine::Combine;
pub use permutate::Permutate;

pub fn select_index<T: Clone>(tgt: &Vec<T>, selected: &Vec<usize>) -> Vec<T> {
    let mut res = vec![];
    selected.iter().for_each(|&i| res.push(tgt[i].clone()));
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_combine() {
        let data = vec![1, 2, 3];
        let val = combine::from_vec_at(&data, 2);
        for item in val {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_permutation() {
        let data = vec![1, 2, 3];
        let val = permutate::from_vec(&data);
        for item in val {
            println!("{:?}", item);
        }
    }
}
