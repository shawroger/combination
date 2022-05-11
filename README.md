# Combination

## What's this

combination is a lib to do math jobs like permutating and combinating data from the list.

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

# Selector

By implementing this trait, then make any type as a selector.

Then using this for the list, it can select value from the list as custom mode.

## Example

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
