use super::{internal::c, select::Selector};

/// # Combine
///
/// a select_list generator
///
/// it can create Vec<SelectList> as combine mode
#[derive(Debug)]
pub struct Combine(usize, usize);

impl Combine {
    /// # new
    ///
    /// group_size must less than element_size
    ///
    /// or it will return an error
    pub fn new(element_size: usize, group_size: usize) -> Self {
        Self(element_size, group_size)
    }
}

impl Selector for Combine {
    /// it will return a list of SelectList
    ///
    /// # Example
    /// ```
    /// use crate::v2::*;
    /// let combine = Combine::new(3, 2);
    /// let select_list = vec![
    ///     SelectList::from([0, 1]),
    ///     SelectList::from([0, 2]),
    ///     SelectList::from([1, 2]),
    /// ];
    /// ```
    /// the two above are same in value
    ///
    fn select_mode(&self) -> Vec<Vec<usize>> {
        c(self.0, self.1).into_iter().map(|v| v.into()).collect()
    }
}

#[cfg(test)]
mod test {

    use super::super::select::Select;
    use super::*;

    #[test]
    fn test_combine_with_select() {
        let str_list = ["hi", "i", "am", "roger", "and", "you"];
        let combine = Combine::new(6, 4);
        let res = str_list.try_select(&combine).unwrap();
        for v in res {
            println!("{:?}", v);
        }
    }
}
