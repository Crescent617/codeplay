use std::ops::{AddAssign, Sub};

// trait BITreeElem = Add + Sub + Default;

pub struct BIT<T>
where
    T: AddAssign + Sub<Output = T> + Default + Copy,
{
    buf: Vec<T>,
}

impl<T> BIT<T>
where
    T: AddAssign + Sub<Output = T> + Default + Copy,
{
    pub fn new(n: usize) -> Self {
        let mut buf = Vec::with_capacity(n + 1);
        for _ in 0..n + 1 {
            buf.push(T::default());
        }
        BIT { buf }
    }

    pub fn update(&mut self, i: usize, delta: T) {
        let mut i = i + 1;
        while i < self.buf.len() {
            self.buf[i] += delta;
            i += i - (i & (i - 1));
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len() - 1
    }

    fn prefix(&self, i: usize) -> T {
        let mut i = i + 1;
        let mut ans = T::default();
        while i > 0 {
            ans += self.buf[i];
            i = i & (i - 1);
        }
        ans
    }

    /// **left** inclusive, **right** exclusive
    pub fn query(&self, left: usize, right: usize) -> T {
        self.prefix(right.saturating_sub(1)) - self.prefix(left.saturating_sub(1))
    }
}
