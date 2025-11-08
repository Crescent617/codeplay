use std::{fmt, mem};

type Compute<T> = Box<dyn FnOnce() -> Stream<T>>;

pub struct NonEmptyStream<T> {
    _head: T,
    _tail: Stream<T>,
    compute_tail: Option<Compute<T>>,
}

pub enum StreamErr {
    TailOnEmpty,
}

pub enum Stream<T> {
    Empty,
    NonEmpty(Box<NonEmptyStream<T>>),
}

impl<T> NonEmptyStream<T> {
    pub fn head(&self) -> &T {
        &self._head
    }

    fn compute(&mut self) {
        self.compute_tail.take().map(|f| self._tail = f());
    }

    pub fn tail(&mut self) -> &mut Stream<T> {
        self.compute();
        &mut self._tail
    }

    pub fn pop_tail(&mut self) -> Stream<T> {
        self.compute();
        mem::replace(&mut self._tail, Empty)
    }
}

use Stream::{Empty, NonEmpty};

impl<T> Stream<T> {
    pub fn new(val: T, f: Compute<T>) -> Self {
        NonEmpty(Box::new(NonEmptyStream {
            _head: val,
            _tail: Empty,
            compute_tail: Some(f),
        }))
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            Empty => None,
            NonEmpty(x) => Some(x.head()),
        }
    }

    pub fn tail(&mut self) -> &mut Stream<T> {
        if let NonEmpty(x) = self {
            x.tail()
        } else {
            panic!()
        }
    }

    pub fn pop_tail(&mut self) -> Stream<T> {
        match self {
            Empty => Empty,
            NonEmpty(x) => x.pop_tail(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            NonEmpty(_) => false,
        }
    }
}

impl<T: 'static> Stream<T> {
    pub fn map<G: Fn(&T) -> T + 'static>(self, f: G) -> Self {
        if let NonEmpty(mut s) = self {
            let (head, tail) = (f(&s._head), s.pop_tail());
            let compute_tail = || tail.map(f);
            Self::new(head, Box::new(compute_tail))
        } else {
            Empty
        }
    }

    pub fn filter<G: Fn(&T) -> bool + 'static>(self, f: G) -> Self {
        if let NonEmpty(mut s) = self {
            let tail = s.pop_tail();
            let head = s._head;
            let filtered = f(&head);
            let compute_tail = || tail.filter(f);

            if filtered {
                Self::new(head, Box::new(compute_tail))
            } else {
                compute_tail()
            }
        } else {
            Empty
        }
    }

    pub fn take(self, num: usize) -> Self {
        if let NonEmpty(mut s) = self {
            let tail = s.pop_tail();
            let head = s._head;
            let compute_tail = move || {
                if num > 1 {
                    tail.take(num - 1)
                } else {
                    Empty
                }
            };
            Self::new(head, Box::new(compute_tail))
        } else {
            Empty
        }
    }
}

pub fn int_stream(i: i32) -> Stream<i32> {
    if i < i32::MAX {
        Stream::new(i, Box::new(move || int_stream(i + 1)))
    } else {
        Empty
    }
}

fn _prime_stream(mut s: Stream<i32>) -> Stream<i32> {
    let i = *s.head().unwrap();
    let tail = s.pop_tail();
    let f = move || _prime_stream(tail.filter(move |x| x % i != 0));
    Stream::new(i, Box::new(f))
}

pub fn prime_stream() -> Stream<i32> {
    _prime_stream(int_stream(2))
}
