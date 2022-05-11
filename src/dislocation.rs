/*!
# All disloation pattern

Just as its name means:

A man wrote n different letters and corresponding n different envelopes.

He packed all the letters into the wrong envelope.

Asked how many kinds of envelopes are packed in the wrong envelope?

*/

use super::{internal::a, Selector};

/**
# All disloation pattern

Just as its name means:

A man wrote different letters and corresponding n different envelopes.

He packed all the letters in the wrong envelope.

Asked how many kinds of envelopes are packed in the wrong envelope?

# Example

```rust, no_run
let a = Dislocation::new(1);
```

It will turn `a` as `[]`

```rust, no_run
let a = Dislocation::new(2);
```

It will turn `a` as `[1, 0]`, means `1` in posotion 0, and `0` in posotion 1

*/
#[derive(Debug)]
pub struct Dislocation(usize);

impl Dislocation {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl Selector for Dislocation {
    fn select_mode(&self) -> Vec<Vec<usize>> {
        a(self.0)
    }
}
