
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<Reverse<i32>>,
    outdated: HashMap<i32, i32>,
    small_size: usize,
    large_size: usize,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
            outdated: HashMap::new(),
            small_size: 0,
            large_size: 0,
        }
    }

    fn insert(&mut self, val: i32) {
        if Some(&val) <= self.small.peek() {
            self.small.push(val);
            self.small_size += 1;
        } else {
            self.large.push(Reverse(val));
            self.large_size += 1;
        }
        self.make_balance();
    }

    fn remove(&mut self, val: i32) {
        if Some(&val) <= self.small.peek() {
            self.small_size -= 1;
        } else {
            self.large_size -= 1;
        }
        *self.outdated.entry(val).or_default() += 1;
    }

    #[inline]
    fn prune(&mut self) {
        while let Some(x) = self.peek_outdated(self.small.peek()) {
            self.small.pop();
            let p = self.outdated.entry(x).or_default();
            *p -= 1;
            if *p == 0 {
                self.outdated.remove(&x);
            }
        }
        while let Some(x) = self.peek_outdated(self.large.peek().map(|x| &x.0)) {
            self.large.pop();
            let p = self.outdated.entry(x).or_default();
            *p -= 1;
            if *p == 0 {
                self.outdated.remove(&x);
            }
        }
    }

    #[inline]
    fn peek_outdated(&self, val: Option<&i32>) -> Option<i32> {
        val.filter(|x| self.outdated.get(x) > Some(&0)).map(|x| *x)
    }

    fn make_balance(&mut self) {
        self.prune();
        if self.small_size > self.large_size + 1 {
            self.small.pop().map(|x| self.large.push(Reverse(x)));
            self.small_size -= 1;
            self.large_size += 1;
        } else if self.small_size < self.large_size {
            self.large.pop().map(|x| self.small.push(x.0));
            self.small_size += 1;
            self.large_size -= 1;
        }
        self.prune();
    }

    fn get_median(&self) -> Option<f64> {
        if (self.small_size + self.large_size) % 2 == 0 {
            self.small
                .peek()
                .zip(self.large.peek())
                .map(|x| (*x.0 as f64 + x.1 .0 as f64) / 2.0)
        } else {
            self.small.peek().map(|x| *x as f64)
        }
    }
}
