use super::{internal::c, select::Selector};

/// # Combine
///
/// a select_list generator
///
/// it can create Vec<Vec<usize>> as combine mode
#[derive(Debug)]
pub struct Combine(usize, usize);

impl Combine {
    pub fn new(element_size: usize, group_size: usize) -> Self {
        Self(element_size, group_size)
    }
}

impl Selector for Combine {
    fn select_mode(&self) -> Vec<Vec<usize>> {
        c(self.0, self.1).into_iter().map(|v| v.into()).collect()
    }
}
