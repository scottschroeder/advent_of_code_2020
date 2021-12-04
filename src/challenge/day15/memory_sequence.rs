use std::num::NonZeroU32;

// Keep in its own module to protect counter and avoid violating invariants
#[derive(Debug)]
pub struct Sequence {
    history: Vec<Option<NonZeroU32>>,
    // Must never be zero, not making this NonZeroU32 because unchecked_add is nightly only
    counter: u32,
    last: u32,
}

impl Sequence {
    pub fn init(starting: &[u32], capacity: usize) -> Sequence {
        let mut s = Sequence {
            history: Vec::with_capacity(capacity),
            counter: 1,
            last: starting[0],
        };
        for x in starting.iter().skip(1) {
            s.push(*x);
        }
        s
    }
    fn push(&mut self, x: u32) {
        self.extend_if_needed(self.last);
        unsafe {
            // counter can never be zero
            self.history[self.last as usize] = Some(NonZeroU32::new_unchecked(self.counter));
        }
        self.counter += 1;
        self.last = x;
    }
    fn age_of(&self, x: u32) -> u32 {
        if let Some(entry) = self.history.get(x as usize) {
            entry.map(|last| self.counter - last.get()).unwrap_or(0)
        } else {
            0
        }
    }
    fn turn(&mut self) -> u32 {
        let turn = self.counter;
        let age = self.age_of(self.last);
        let last = self.last;
        self.push(age);
        log::trace!("turn: {} - last: {} speak: {}", turn, last, age);
        age
    }
    fn extend_if_needed(&mut self, x: u32) {
        let new_len = x as usize + 1;
        if new_len > self.history.len() {
            self.history.resize(new_len, None)
        }
    }
}

impl Iterator for Sequence {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.turn())
    }
}
