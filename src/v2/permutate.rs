use super::{internal::p, select::Selector};

/// # Permutate
///
/// a select_list generator
///
/// it can create Vec<Vec<usize>> as permutate mode
#[derive(Debug)]
pub struct Permutate(usize, usize);

impl Permutate {
    /// # new
    ///
    /// create a `Permutate` struct
    pub fn new(element_size: usize, group_size: usize) -> Self {
        Self(element_size, group_size)
    }
}

impl Selector for Permutate {
    /// it will return a list of Vec<usize>
    ///
    /// # Example
    /// ```
    /// let permutate = Permutate::new(3, 2);
    /// let select_list = vec![
    ///     [0, 1], [1, 0], [0, 2], [2, 0], [1, 2], [2, 1]
    /// ];
    /// ```
    /// the two above are same in value
    ///
    fn select_mode(&self) -> Vec<Vec<usize>> {
        p(self.0, self.1).into_iter().map(|v| v.into()).collect()
    }
}
