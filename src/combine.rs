use crate::select_index;

fn __wrap(tgt: &Vec<usize>, size: usize) -> Vec<(Vec<usize>, usize)> {
    let mut res: Vec<(Vec<usize>, usize)> = Vec::new();

    if size < 1 {
        return res;
    }
    if size == 1 {
        for i in 0..tgt.len() {
            res.push((vec![tgt[i]], i));
        }
        return res;
    }

    let temp = __wrap(tgt, size - 1);
    for item in temp {
        let (presize_arr, index) = item;
        if index < tgt.len() {
            for j in (index + 1)..tgt.len() {
                let mut presize_arr_copy = presize_arr.clone();
                presize_arr_copy.push(tgt[j]);
                res.push((presize_arr_copy, j));
            }
        }
    }

    res
}

pub fn index(size: usize, combine_len: usize) -> Vec<Vec<usize>> {
    let list = (0..size).into_iter().collect();
    let mut result: Vec<Vec<usize>> = Vec::new();
    if size < combine_len {
        return result;
    }
    let temp = __wrap(&list, combine_len);
    for i in temp {
        result.push(i.0);
    }
    result
}

/// Get combination data from a vec
///
/// * for example
/// ``` rust
/// use combination::*;
/// let val = combine::from_vec_at(&vec![10, 20, 30, 40], 2);
/// for item in val {
///     println!("{:?}", item);
/// }
/// ```
///
/// * and will get:
///
///```text
/// [10, 20]
/// [10, 30]
/// [10, 40]
/// [20, 30]
/// [20, 40]
/// [30, 40]
///```
pub fn from_vec_at<T: Clone>(tgt: &Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();
    let temp = index(tgt.len(), size);
    for item in &temp {
        let temp = select_index(&tgt, item);
        res.push(temp);
    }
    res
}

/// Get all combination result from a vec
///
/// * for example
/// ``` rust
/// use combination::*;
/// let val = combine::from_vec(&vec![10, 20, 30]);
/// for item in val {
///     println!("{:?}", item);
/// }
/// ```
///
/// * and will get:
///
///```text
/// [10]
/// [20]
/// [30]
/// [10, 20]
/// [10, 30]
/// [20, 30]
/// [10, 20, 30]
///```
pub fn from_vec<T: Clone>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();

    (0..list.len()).for_each(|n| from_vec_at(&list, n).into_iter().for_each(|v| res.push(v)));
    res.push(list.clone());
    res
}

pub trait Combine<T: Clone> {
    fn combine(&self) -> Vec<Vec<T>>;
    fn combine_at(&self, size: usize) -> Vec<Vec<T>>;
}

impl<T: Clone> Combine<T> for Vec<T> {
    fn combine(&self) -> Vec<Vec<T>> {
        from_vec(self)
    }

    fn combine_at(&self, size: usize) -> Vec<Vec<T>> {
        from_vec_at(self, size)
    }
}
