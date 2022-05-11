/**
# Selector

by implement this trait, then make any type as a selector

pass this for the list, it can select value as this mode

# Example

```no_run
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
      println!("{:?}", v);
   }
}
```

it will get result as below:

```no_run
[
   ["how", "how", "how"],
   ["are", "are", "are"],
   ["u", "u", "u"]
]
```
*/
pub trait Selector {
    fn select_mode(&self) -> Vec<Vec<usize>>;
}

/**
# Select

This trait can allow any type to select any ref from itself.

Whatever implements [Selector] can be used to select item that implements [Select]
*/
pub trait Select<T> {
    /// select a list from one `SelectList`
    fn select_one_list(&self, select_list: &[usize]) -> Vec<Option<&T>>;

    /// Select a list of values
    ///
    /// Ff index is not in the list, the value in this index position will be set as None
    fn select<U: Selector>(&self, select_group: &U) -> Vec<Vec<Option<&T>>> {
        select_group
            .select_mode()
            .into_iter()
            .map(|s| self.select_one_list(&s))
            .collect()
    }

    /// Select a list of values
    ///
    /// Ff index is not in the list, it will return errortype
    fn try_select<U: Selector>(&self, select_group: &U) -> Result<Vec<Vec<&T>>, InsufficientSize> {
        let res: Vec<Result<Vec<&T>, usize>> = select_group
            .select_mode()
            .into_iter()
            .map(|list| {
                match self
                    .select_one_list(&list)
                    .iter()
                    .enumerate()
                    .find(|(_, v)| v.is_none())
                {
                    Some((index, _)) => Err(index),
                    None => Ok(self
                        .select_one_list(&list)
                        .into_iter()
                        .map(|v| v.unwrap())
                        .collect::<Vec<&T>>()),
                }
            })
            .collect();

        match res.iter().enumerate().find(|(_, v)| v.is_err()) {
            Some((index, _)) => Err(InsufficientSize::new(index)),
            None => Ok(res.into_iter().map(|list| list.unwrap()).collect()),
        }
    }
}

/// # InsufficientSize
///
/// It is an **error** value which means that the index of `select_list` is over the element list length
#[derive(Debug)]
#[allow(dead_code)]
pub struct InsufficientSize {
    index: usize,
}

impl InsufficientSize {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

macro_rules! select_impl {
    ($b:ident, $t:ty) => {
        impl<$b> Select<$b> for $t {
            fn select_one_list(&self, select_list: &[usize]) -> Vec<Option<&T>> {
                select_list.iter().map(|&i| self.get(i)).collect()
            }
        }
    };

    ($b:ident, $($t:ty),*) => {
       $(select_impl!($b, $t);)*
    };
}

// Implement Select trait
select_impl!(T, &[T], Vec<T>, [T]);

impl<T, const N: usize> Select<T> for [T; N] {
    fn select_one_list(&self, select_list: &[usize]) -> Vec<Option<&T>> {
        select_list.iter().map(|&i| self.get(i)).collect()
    }
}
