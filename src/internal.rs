//! # Internal functions
//!
//! This module provides internal functions which calcuate different type of combinations and arrangements.
//!

fn _c(starter: &Vec<usize>, group: usize) -> Vec<(Vec<usize>, usize)> {
    let starter_len = starter.len();
    let mut res = vec![];

    if starter_len < group || group < 1 {
        return res;
    }

    if group == 1 {
        return (0..starter_len).map(|i| (vec![starter[i]], i)).collect();
    }

    _c(starter, group - 1).into_iter().for_each(|(arr, index)| {
        ((index + 1)..starter_len).into_iter().for_each(|i| {
            res.push(([arr.clone(), vec![starter[i]]].concat(), i));
        })
    });

    res
}

fn _p(size: usize) -> Vec<Vec<usize>> {
    if size <= 1 {
        return vec![vec![0]];
    }

    if size <= 2 {
        return vec![vec![0, 1], vec![1, 0]];
    }

    let mut res = vec![];

    _p(size - 1).into_iter().for_each(|list| {
        (0..size)
            .into_iter()
            .map(|n| size - n - 1)
            .for_each(|index| {
                let mut temp = vec![];

                list.iter().for_each(|&i| {
                    if temp.len() == index {
                        temp.push(size - 1);
                    }
                    temp.push(i);
                });

                if temp.len() == index {
                    temp.push(size - 1);
                }

                res.push(temp);
            });
    });

    res
}

pub fn c(element_size: usize, group: usize) -> Vec<Vec<usize>> {
    let starter = (0..element_size).into_iter().collect();
    _c(&starter, group).into_iter().map(|(v, _)| v).collect()
}

pub fn p(element_size: usize, group_size: usize) -> Vec<Vec<usize>> {
    use super::select::Select;
    let mut res = vec![];
    let permutate = _p(group_size);
    let starter = (0..element_size).into_iter().collect();

    // no need check unwrap
    // fn _c do it
    _c(&starter, group_size).into_iter().for_each(|(list, _)| {
        permutate.iter().for_each(|select_list| {
            res.push(
                list.clone()
                    .select_one_list(select_list)
                    .into_iter()
                    .map(|v| *v.unwrap())
                    .collect(),
            );
        });
    });

    res
}

fn _a_unaccept(list: &Vec<usize>) -> bool {
    !list.iter().enumerate().any(|(v, &i)| v == i)
}

pub fn a(size: usize) -> Vec<Vec<usize>> {
    _p(size)
        .into_iter()
        .filter(|list| _a_unaccept(list))
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_c() {
        let res = c(5, 3);
        for v in res {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_p() {
        let res = p(2, 3);
        for v in res {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_a() {
        let res = a(4);
        for v in res {
            println!("{:?}", v);
        }
    }
}
