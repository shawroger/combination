# Combination

## what it is

combination is a lib to do math jobs like permutate and combinate data from vec.

## example

```rust
extern crate combination;
use combination::*;

#[test]
#[cfg(test)]
fn test_permutation() {
    let data = vec![10, 20, 30, 40];
    let val = permutate::from_vec(&data);
    for item in val {
       println!("{:?}", item);
    }
}
```

# API

## combine

Get combination data from a vec

- for example

```rust
use combination::*;
let val = combine::from_vec(&vec![10, 20, 30, 40], 2);
for item in val {
    println!("{:?}", item);
}
```

- and will get:

```
[10, 20]
[10, 30]
[10, 40]
[20, 30]
[20, 40]
[30, 40]
```

## permutate

Get permutation data from a vec

- for example

```rust
extern crate combination;
use combination::*;
let val = permutate::from_vec(&vec![10, 20, 30, 40]);
for item in val {
    println!("{:?}", item);
}
```

- and will get:

```
    [30, 10, 40, 20]
    [30, 10, 20, 40]
    [40, 10, 30, 20]
    [10, 40, 30, 20]
    [10, 30, 40, 20]
    [10, 30, 20, 40]
    [40, 10, 20, 30]
    [10, 40, 20, 30]
    [10, 20, 40, 30]
    [10, 20, 30, 40]
    [40, 30, 20, 10]
    [30, 40, 20, 10]
    [30, 20, 40, 10]
    [30, 20, 10, 40]
    [40, 20, 30, 10]
    [20, 40, 30, 10]
    [20, 30, 40, 10]
    [20, 30, 10, 40]
    [40, 20, 10, 30]
    [20, 40, 10, 30]
    [20, 10, 40, 30]
    [20, 10, 30, 40]
```

# Recommend v2 module

In 2022, this package is using `edition 2021` now and do more work.

In v2, no longer needs `Clone` trait.

And Permutate now allow two params.

What is more, it use Optional value and any input won’t occur error.

Anyway, **old api has not changed**, don't worry.

Any you are going to use v2 module, just add `v2` in the feature, and it is used by default.

## Example

```rust
use combination::v2::\*;
let str_list = ["hi", "i", "am", "roger", "and", "you"];
let combine = Combine::new(6, 4);
let res = str_list.try_select(&combine).unwrap();

for v in res {
    println!("{:?}", v);
}
```

As long as use two traits Select and Selector it will work.

V2 module provides two structs which implement `Selector` trait, they are `Combine` and `Permutate`

By using them with trait `Select`, type as `&[T], Vec<T>, [T]` will be able to be selected.

## Selector

By implementing this trait, then make any type as a selector.

Then using this for the list, it can select value from the list as custom mode.

### Example

```rust
struct CustomSelector;

impl Selector for CustomSelector {
    fn select_mode(&self) -> Vec<Vec<usize>> {
        vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 2, 2]]
    }
}
fn test_custom_selector() {
    let str_list = ["how", "are", "u"];
    let custom_selector = CustomSelector;
    let res = str_list.try_select(&custom_selector).unwrap();

    for v in res {
        println!("{:#?}", v);
    }
}
```

it will be

```rust
[
    ["how", "how", "how"],
    ["are", "are", "are"],
    ["u", "u", "u"]
]
```
