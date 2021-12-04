use std::num::NonZeroUsize;

// Keep in its own module to protect counter and avoid violating invariants
#[derive(Debug)]
pub struct Sequence {
    history: Vec<Option<NonZeroUsize>>,
    // Must never be zero, not making this NonZeroUsize because unchecked_add is nightly only
    counter: usize,
    last: usize,
}

impl Sequence {
    pub fn init(starting: &[u32], capacity: usize) -> Sequence {
        let mut s = Sequence {
            history: Vec::with_capacity(capacity),
            counter: 1,
            last: starting[0] as usize,
        };
        for x in starting.iter().skip(1) {
            s.push(*x as usize);
        }
        s
    }
    fn push(&mut self, x: usize) {
        self.extend_if_needed(self.last);
        unsafe {
            // counter can never be zero
            self.history[self.last] = Some(NonZeroUsize::new_unchecked(self.counter));
        }
        self.counter += 1;
        self.last = x;
    }
    fn age_of(&self, x: usize) -> usize {
        if let Some(entry) = self.history.get(x) {
            entry.map(|last| self.counter - last.get()).unwrap_or(0)
        } else {
            0
        }
    }
    fn turn(&mut self) -> usize {
        let turn = self.counter;
        let age = self.age_of(self.last);
        let last = self.last;
        self.push(age);
        log::trace!("turn: {} - last: {} speak: {}", turn, last, age);
        age
    }
    fn extend_if_needed(&mut self, x: usize) {
        let new_len = x + 1;
        if new_len > self.history.len() {
            self.history.resize(new_len, None)
        }
    }
}

impl Iterator for Sequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.turn())
    }
}
