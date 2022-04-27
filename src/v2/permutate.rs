use super::{internal::p, select::Selector};

/// # Permutate
///
/// a select_list generator
///
/// it can create Vec<SelectList> as permutate mode
#[derive(Debug)]
pub struct Permutate(usize, usize);

impl Selector for Permutate {
    /// it will return a list of SelectList
    ///
    /// # Example
    /// ```
    /// let permutate = Permutate::new(3, 2);
    /// let select_list = vec![
    ///     SelectList::from([0, 1]),
    ///     SelectList::from([0, 2]),
    ///     SelectList::from([1, 2]),
    /// ];
    /// ```
    /// the two above are same in value
    ///
    fn select_mode(&self) -> Vec<Vec<usize>> {
        p(self.0, self.1).into_iter().map(|v| v.into()).collect()
    }
}

impl Permutate {
    /// # new
    ///
    /// group_size must less than element_size
    ///
    /// or it will return an error
    pub fn new(element_size: usize, group_size: usize) -> Self {
        Self(element_size, group_size)
    }
}
