//!  # What it is
//!  combination is a lib to do math jobs like permutate and combinate data from vec.
//!
//! ```rust
//! use combination::*;
//! fn test_permutation() {
//!     let val = permutate(&vec![10, 20, 30, 40]);
//!     for item in val {
//!        println!("{:?}", item);
//!     }
//! }
//! ```
//!
//!
//!
fn example_vec(size: usize) -> Vec<usize> {
    let mut provide_temp: Vec<usize> = Vec::new();
    for i in 0..size {
        provide_temp.push(i);
    }
    provide_temp
}

fn vector_mapper<T: Clone>(target_vec: &Vec<T>, map_from_vec: &Vec<usize>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    for index in map_from_vec {
        let item = target_vec[*index].clone();
        result.push(item);
    }
    result
}

fn impl_combine(target_vec: &Vec<usize>, size: usize) -> Vec<(Vec<usize>, usize)> {
    let mut result: Vec<(Vec<usize>, usize)> = Vec::new();
    if size == 1 {
        for i in 0..target_vec.len() {
            result.push((vec![target_vec[i]], i));
        }
        return result;
    } else if size > 1 {
        let presize_result = impl_combine(target_vec, size - 1);
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

fn combine_index(size: usize, combine_len: usize) -> Vec<Vec<usize>> {
    if size < combine_len {
        panic!("Math Error: can not combine elements more than all");
    }
    let example_vec = example_vec(size);
    let mut result: Vec<Vec<usize>> = Vec::new();
    let combine_result = impl_combine(&example_vec, combine_len);
    for i in combine_result {
        result.push(i.0);
    }
    result
}

/// Get combination data from a vec
///
/// * for example
/// ``` rust
/// use combination::*;
/// let val = combine(&vec![10, 20, 30, 40], 2);
/// for item in val {
///     println!("{:?}", item);
/// }
/// ```
///
/// * and will get:
///
/// [10, 20]
/// [10, 30]
/// [10, 40]
/// [20, 30]
/// [20, 40]
/// [30, 40]
///
pub fn combine<T: Clone>(target_vec: &Vec<T>, combine_len: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let permutate_vec = combine_index(target_vec.len(), combine_len);
    for item in &permutate_vec {
        let temp = vector_mapper(&target_vec, item);
        result.push(temp);
    }
    result
}

/// Get all combination result from a vec
///
/// * for example
/// ``` rust
/// use combination::*;
/// let val = combine_all(&vec![10, 20, 30]);
/// for item in val {
///     println!("{:?}", item);
/// }
/// ```
///
/// * and will get:
///
/// [10]
/// [20]
/// [30]
/// [10, 20]
/// [10, 30]
/// [20, 30]
/// [10,20, 30]
pub fn combine_all<T: Clone>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();

    (0..list.len()).for_each(|n| combine(&list, n).into_iter().for_each(|v| res.push(v)));
    res.push(list.clone());
    res
}

fn permutate_index(cut_slice: usize) -> Vec<Vec<usize>> {
    if cut_slice <= 1 {
        vec![vec![0]]
    } else if cut_slice <= 2 {
        vec![vec![0, 1], vec![1, 0]]
    } else {
        let mut result: Vec<Vec<usize>> = Vec::new();
        let presize_arr = permutate_index(cut_slice - 1);
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
/// use combination::*;
/// let val = permutate(&vec![10, 20, 30, 40]);
/// for item in val {
///     println!("{:?}", item);
/// }
/// ```
///
/// * and will get:
///
///
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
///

pub fn permutate<T: Clone>(target_vec: &Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let permutate_vec = permutate_index(target_vec.len());
    for item in &permutate_vec {
        let temp = vector_mapper(target_vec, item);
        result.push(temp);
    }
    result
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_permutation() {
        let data = vec![1, 2, 3];
        let val = permutate(&data);
        for item in val {
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_combine() {
        let data = vec![1, 2, 3];
        let val = combine(&data, 2);
        for item in val {
            println!("{:?}", item);
        }
    }
}
