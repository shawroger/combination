extern crate combination;
use combination::permutate::*;
use combination::combine::*;

#[test]
#[cfg(test)]
fn test_permutation() {
    let data = vec![10, 20, 30, 40];
    let val = permutate_vec(&data);
    for item in val {
        println!("{:?}", item);
    }
}


#[test]
#[cfg(test)]
fn test_combine() {
    let data = vec![10, 20, 30, 40];
    let val = combine_vec(&data, 2);
    for item in val {
        println!("/// {:?}", item);
    }
}
