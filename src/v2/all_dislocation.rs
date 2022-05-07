//! # All disloation pattern
//!
//! just as its name means:
//!
//! A man wrote n different letters and corresponding n different envelopes.
//!
//! He packed all the n letters in the wrong envelope.
//!
//! Asked how many kinds of envelopes are packed in the wrong envelope?
//!

use super::{internal::a, Selector};

/// # All disloation pattern
///
/// just as its name means:
///
/// A man wrote n different letters and corresponding n different envelopes.
///
/// He packed all the n letters in the wrong envelope.
///
/// Asked how many kinds of envelopes are packed in the wrong envelope?
///
/// # Example
///
/// ```rust
/// let a = AllDislocation::new(1);
/// ```
///
/// it will turn `a` as `[]`
///
/// ```rust
/// let a = AllDislocation::new(2);
/// ```
///
/// it will turn `a` as `[1, 0]`, means `1` in posotion 0, and `0` in posotion 1
///
#[derive(Debug)]
pub struct AllDislocation(usize);

impl AllDislocation {
    /// # new
    ///
    /// create an `AllDislocation` struct
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl Selector for AllDislocation {
    /// it will return a list of Vec<usize>
    ///
    /// # Example
    /// ```
    /// use crate::v2::*;
    /// let combine = Combine::new(3, 2);
    /// let select_list = vec![
    ///      [0, 1], [0, 2], [1, 2],
    /// ];
    /// ```
    /// the two above are same in value
    ///
    fn select_mode(&self) -> Vec<Vec<usize>> {
        a(self.0)
    }
}

#[cfg(test)]
mod test {

    use super::super::select::Select;
    use super::*;

    #[test]
    fn test() {
        let str_list = ["hi", "i", "am", "roger", "and", "you"];
        let combine = AllDislocation::new(6);
        let res = str_list.try_select(&combine).unwrap();
        for v in res {
            println!("{:?}", v);
        }
    }
}
