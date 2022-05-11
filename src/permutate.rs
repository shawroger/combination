use super::{internal::p, select::Selector};

/// # Permutate
///
/// It can create Vec<Vec<usize>> as a permutating mode.
#[derive(Debug)]
pub struct Permutate(usize, usize);

impl Permutate {
    pub fn new(element_size: usize, group_size: usize) -> Self {
        Self(element_size, group_size)
    }
}

impl Selector for Permutate {
    fn select_mode(&self) -> Vec<Vec<usize>> {
        p(self.0, self.1).into_iter().map(|v| v.into()).collect()
    }
}
