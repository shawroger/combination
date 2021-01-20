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
    let val = permutate(&data);
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
let val = combine(&vec![10, 20, 30, 40], 2);
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
let val = permutate(&vec![10, 20, 30, 40]);
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
