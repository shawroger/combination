use crate::select_index;

pub fn index(size: usize) -> Vec<Vec<usize>> {
    if size <= 1 {
        return vec![vec![0]];
    }

    if size <= 2 {
        return vec![vec![0, 1], vec![1, 0]];
    }
    let mut res: Vec<Vec<usize>> = Vec::new();
    let index_arr = index(size - 1);
    for item in index_arr {
        for i in 0..size {
            let mut temp: Vec<usize> = Vec::new();
            for j in &item {
                if temp.len() == i {
                    temp.push(size - 1);
                }
                temp.push(*j);
            }
            if temp.len() == i {
                temp.push(size - 1);
            }
            res.push(temp);
        }
    }
    res
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

pub fn from_vec<T: Clone>(tgt: &Vec<T>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = Vec::new();
    let index_arr = index(tgt.len());
    for item in &index_arr {
        let temp = select_index(tgt, item);
        res.push(temp);
    }
    res
}

pub trait Permutate<T: Clone> {
    fn permutate(&self) -> Vec<Vec<T>>;
}

impl<T: Clone> Permutate<T> for Vec<T> {
    fn permutate(&self) -> Vec<Vec<T>> {
        from_vec(self)
    }
}
