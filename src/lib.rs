/*!
# Combination
combination is a lib to do math jobs like permutate and combinate data from vec.

# Example
```
use combination::v2::*;

fn main() {
    let combine = Combine::new(6, 4);
    let list = ["hi", "i", "am", "roger", "and", "you", "?"];
    let res = list.try_select(&combine).unwrap();
    for v in res {
        println!("{:?}", v);
    }
}

```

As long as use two traits `Select` and `Selector` it will work.

Internal module provides two structs which implement `Selector` trait, they are `Combine` and `Permutate`

By using them with trait `Select`, type as `&[T], Vec<T>, [T; N]` will be able to be selected.

*/

mod internal;

mod combine;
mod dislocation;
mod permutate;
mod select;

pub use combine::Combine;
pub use dislocation::Dislocation;
pub use permutate::Permutate;
pub use select::{InsufficientSize, Select, Selector};
