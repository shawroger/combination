//!  # What it is
//!  math_permutation is a lib to do math jobs
//!
//!  it can do something like permutation and combination.
//!
//!   ```rust
//!
//! extern crate math_permutation;
//! use math_permutation::permutation::*;
//!
//! #[test]
//! #[cfg(test)]
//! fn test_permutation() {
//!     let data = vec![10, 20, 30, 40];
//!     let val = permutate_vec(&data);
//!     for item in val {
//!        println!("{:?}", item);
//!     }
//! }
//! ```
//!
//!
//! 
#![crate_name = "combination"]
#![deny(warnings)]
#[allow(dead_code)]
fn get_example_vec(size: usize) -> Vec<usize> {
    let mut provide_temp: Vec<usize> = Vec::new();
    for i in 0..size {
        provide_temp.push(i);
    }
    provide_temp
}

fn map_from_vec<T: Clone + Copy>(target_vec: &Vec<T>, map_from_vec: &Vec<usize>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let target_vec_owned = target_vec.to_vec();

    for index in map_from_vec {
        let item = target_vec_owned[*index];
        result.push(item);
    }
    result
}

pub mod combine {
    use super::*;
    fn get_combination_impl(target_vec: &Vec<usize>, size: usize) -> Vec<(Vec<usize>, usize)> {
        let mut result: Vec<(Vec<usize>, usize)> = Vec::new();
        if size == 1 {
            for i in 0..target_vec.len() {
                result.push((vec![target_vec[i]], i));
            }
            return result;
        } else if size > 1 {
            let presize_result = get_combination_impl(target_vec, size - 1);
            for item in presize_result {
                let (presize_arr, index) = item;
                if index < target_vec.len() {
                    for j in (index + 1)..target_vec.len() {
                        let mut presize_arr_copy = presize_arr.clone();
                        presize_arr_copy.push(target_vec[j]);
                        result.push((presize_arr_copy, j));
                    }
                }
            }
            return result;
        }
        result
    }

    fn get_combination(size: usize, combine_len: usize) -> Vec<Vec<usize>> {
        if size < combine_len {
            panic!("Math Error: can not combine elements more than all");
        }
        let example_vec = get_example_vec(size);
        let mut result: Vec<Vec<usize>> = Vec::new();
        let combine_result = get_combination_impl(&example_vec, combine_len);
        for i in combine_result {
            result.push(i.0);
        }
        result
    }

    /// Get combination data from a vec
    ///
    /// * for example
    /// ``` rust
    /// let val = combination_vec(&vec![10, 20, 30, 40], 2);
    /// for item in val {
    ///     println!("{:?}", item);
    /// }
    /// ```
    /// 
    /// * and will get:
    /// 
    ///``` rust
    /// [10, 20]
    /// [10, 30]
    /// [10, 40]
    /// [20, 30]
    /// [20, 40]
    /// [30, 40]
    /// ```
    pub fn combine_vec<T: Clone + Copy>(target_vec: &Vec<T>, combine_len: usize) -> Vec<Vec<T>> {
        let mut result: Vec<Vec<T>> = Vec::new();
        let permutate_vec = get_combination(target_vec.len(), combine_len);
        for item in &permutate_vec {
            let temp = map_from_vec(&target_vec, item);
            result.push(temp);
        }
        result
    }
}

pub mod permutate {
    use super::*;

    fn get_permutation(cut_slice: usize) -> Vec<Vec<usize>> {
        if cut_slice <= 1 {
            vec![vec![0]]
        } else if cut_slice <= 2 {
            vec![vec![0, 1], vec![1, 0]]
        } else {
            let mut result: Vec<Vec<usize>> = Vec::new();
            let presize_arr = get_permutation(cut_slice - 1);
            for item in &presize_arr {
                for index in 0..cut_slice {
                    let mut temp: Vec<usize> = Vec::new();
                    for sub_item in item {
                        if temp.len() == index {
                            temp.push(cut_slice - 1);
                        }
                        temp.push(*sub_item);
                    }
                    if temp.len() == index {
                        temp.push(cut_slice - 1);
                    }
                    result.push(temp);
                }
            }
            result
        }
    }

    /// Get permutation data from a vec
    ///
    /// * for example
    /// ``` rust
    /// let val = permutate_vec(&vec![10, 20, 30, 40]);
    /// for item in val {
    ///     println!("{:?}", item);
    /// }
    /// ```
    /// 
    /// * and will get:
    /// 
    /// ``` rust
    /// [30, 10, 40, 20]
    /// [30, 10, 20, 40]
    /// [40, 10, 30, 20]
    /// [10, 40, 30, 20]
    /// [10, 30, 40, 20]
    /// [10, 30, 20, 40]
    /// [40, 10, 20, 30]
    /// [10, 40, 20, 30]
    /// [10, 20, 40, 30]
    /// [10, 20, 30, 40]
    /// [40, 30, 20, 10]
    /// [30, 40, 20, 10]
    /// [30, 20, 40, 10]
    /// [30, 20, 10, 40]
    /// [40, 20, 30, 10]
    /// [20, 40, 30, 10]
    /// [20, 30, 40, 10]
    /// [20, 30, 10, 40]
    /// [40, 20, 10, 30]
    /// [20, 40, 10, 30]
    /// [20, 10, 40, 30]
    /// [20, 10, 30, 40]
    /// ```

    pub fn permutate_vec<T: Clone + Copy>(target_vec: &Vec<T>) -> Vec<Vec<T>> {
        let mut result: Vec<Vec<T>> = Vec::new();
        let permutate_vec = get_permutation(target_vec.len());
        for item in &permutate_vec {
            let temp = map_from_vec(&target_vec, item);
            result.push(temp);
        }
        result
    }
}
