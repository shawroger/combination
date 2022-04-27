/// # InsufficientSize
///
/// it means that the index of select_list is over the element list length
#[derive(Debug)]
pub struct InsufficientSize;

/// # Select
///
/// this trait can allow any type select any ref from itself
///
/// whatever implements `Selector` can be used to select items from whatever implements `Select`
pub trait Select<T> {
    /// select a list from one `SelectList`
    fn select_one_list(&self, select_list: &[usize]) -> Vec<Option<&T>>;

    /// select a list of values
    ///
    /// if index is not in the list
    ///
    /// it will set as None
    ///
    /// works as `map(|val| self.select(val))`
    fn select<U: Selector>(&self, select_group: &U) -> Vec<Vec<Option<&T>>> {
        select_group
            .select_mode()
            .into_iter()
            .map(|s| self.select_one_list(&s))
            .collect()
    }

    /// select a list of values
    ///
    /// if index is not in the list
    ///
    /// it will return errortype
    fn try_select<U: Selector>(&self, select_group: &U) -> Result<Vec<Vec<&T>>, InsufficientSize> {
        let res: Vec<Result<Vec<&T>, ()>> = select_group
            .select_mode()
            .into_iter()
            .map(|list| {
                if self.select_one_list(&list).iter().any(|v| v.is_none()) {
                    Err(())
                } else {
                    Ok(self
                        .select_one_list(&list)
                        .into_iter()
                        .map(|v| v.unwrap())
                        .collect::<Vec<&T>>())
                }
            })
            .collect();

        if res.iter().any(|list| list.is_err()) {
            return Err(InsufficientSize);
        }

        Ok(res.into_iter().map(|list| list.unwrap()).collect())
    }
}

/// # Selector
///
/// by implement this trait, then make any type as a selector
///
/// pass this for the list, it can select value as this mode
///
/// # Example
/// ```
///struct CustomSelector;
///
///impl Selector for CustomSelector {
///   fn select_mode(&self) -> Vec<Vec<usize>> {
///       vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 2, 2]]
///   }
///}
///fn test_custom_selector() {
///   let str_list = ["how", "are", "u"];
///   let custom_selector = CustomSelector;
///   let res = str_list.try_select(&custom_selector).unwrap();
///
///   for v in res {
///      println!("{:#?}", v);
///   }
///}
///```
///
/// it will be
///
///
/// ```
/// [
///     ["how", "how", "how"],
///     ["are", "are", "are"],
///     ["u", "u", "u"]
/// ]
///
/// ```
pub trait Selector {
    fn select_mode(&self) -> Vec<Vec<usize>>;
}

macro_rules! _impl_select_for_list {
    ($b:ident, $t:ty) => {
        impl<$b> Select<$b> for $t {
            fn select_one_list(&self, select_list: &[usize]) -> Vec<Option<&T>> {
                select_list.iter().map(|&i| self.get(i)).collect()
            }
        }
    };

    ($b:ident, $($t:ty),*) => {
       $(_impl_select_for_list!($b, $t);)*
    };
}

// implement Select trait
_impl_select_for_list!(T, &[T], Vec<T>, [T]);

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_select_from_str_list() {
        let str_list = ["hi", "i", "am", "roger", "and", "you"];
        let select_list = [1, 2, 3, 7, 2, 3, 0];
        let res = str_list.select_one_list(&select_list);

        for v in res {
            println!("{:#?}", v);
        }
    }

    struct CustomSelector;

    impl Selector for CustomSelector {
        fn select_mode(&self) -> Vec<Vec<usize>> {
            vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 2, 2]]
        }
    }

    #[test]
    fn test_custom_selector() {
        let str_list = ["hi", "i", "am", "roger", "and", "you"];
        let custom_selector = CustomSelector;
        let res = str_list.try_select(&custom_selector).unwrap();

        println!("{:#?}", res);
    }
}
