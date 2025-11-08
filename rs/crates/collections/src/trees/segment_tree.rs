#![allow(dead_code)]

use std::{cell::RefCell, mem, ptr::NonNull, rc::Rc, vec};

use mem::size_of;

pub struct SegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    buf: Vec<Option<T>>,
    size: usize,
    offset: usize,
    _op: F,
}

impl<T: Clone, F> SegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    pub fn new(size: usize, op: F) -> Self {
        Self::cons(vec![], size, op)
    }

    pub fn from(v: Vec<T>, op: F) -> Self {
        let n = v.len();
        Self::cons(v, n, op)
    }

    fn cons(data: Vec<T>, size: usize, op: F) -> Self {
        let bit_num = mem::size_of::<usize>() * 8;
        let offset = 1 << (bit_num - size.leading_zeros() as usize);
        debug_assert!(offset >= size);

        let mut buf = Vec::with_capacity(offset * 2);
        for _ in 0..offset * 2 {
            buf.push(None);
        }

        let mut st = SegTree {
            buf,
            size,
            offset,
            _op: op,
        };

        if !data.is_empty() {
            for (i, x) in data.into_iter().enumerate() {
                st.buf[offset + i] = Some(x);
            }
            for i in (1..offset).rev() {
                st.buf[i] = st.combine(&st.buf[i * 2], &st.buf[i * 2 + 1]);
            }
        }

        st
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn combine(&self, a: &Option<T>, b: &Option<T>) -> Option<T> {
        if let (Some(x), Some(y)) = (a, b) {
            let op = &self._op;
            Some(op(x, y))
        } else if a.is_some() || b.is_some() {
            a.clone().or(b.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, idx: usize, val: T) {
        let mut i = idx + self.offset;
        self.buf[i] = Some(val);

        while i > 1 {
            i >>= 1;
            self.buf[i] = self.combine(&self.buf[i * 2], &self.buf[i * 2 + 1]);
        }
    }

    /// **left** inclusive, **right** exclusive
    pub fn query(&self, left: usize, right: usize) -> Option<T> {
        if left >= right || right > self.size {
            return None;
        }

        let (mut l, mut r) = (left + self.offset, right + self.offset - 1);
        let mut ans = self.buf[l].clone();
        l += 1;

        while l <= r {
            if l % 2 == 1 {
                ans = self.combine(&ans, &self.buf[l]);
                l += 1;
            }
            if r % 2 == 0 {
                ans = self.combine(&ans, &self.buf[r]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }

        ans
    }
}

pub struct PstSegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    roots: Vec<Rc<Node<T>>>,
    _op: F,
    begin: usize,
    end: usize,
}

type Edge<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    size: usize,
    begin: usize,
    end: usize,
    left: Edge<T>,
    right: Edge<T>,
}

impl<T> Node<T> {
    fn new(val: T, begin: usize, end: usize) -> Self {
        Self {
            val,
            size: (begin == end) as usize,
            begin,
            end,
            left: None,
            right: None,
        }
    }

    fn set_left(&mut self, left: Edge<T>) {
        left.map(|x| {
            self.size += x.size;
            self.left.replace(x);
        });
    }

    fn set_right(&mut self, right: Edge<T>) {
        right.map(|x| {
            self.size += x.size;
            self.right.replace(x);
        });
    }

    fn wrap(self) -> Edge<T> {
        Some(Rc::new(self))
    }
}

impl<T: Ord + Copy, F> PstSegTree<T, F>
where
    F: Fn(&T, &T) -> T,
{
    pub fn new(begin: usize, end: usize, op: F) -> Self {
        Self {
            roots: vec![],
            begin,
            end,
            _op: op,
        }
    }

    fn combine(&self, a: &T, b: &T) -> T {
        let f = &self._op;
        f(a, b)
    }

    pub fn insert(&mut self, key: usize, val: T) {
        let mut new_root = Node::new(val, self.begin, self.end);
        self.insert_node(&mut new_root, self.roots.last(), key, val);
        self.roots.push(Rc::new(new_root));
    }

    fn insert_node(&self, new_nd: &mut Node<T>, node: Option<&Rc<Node<T>>>, key: usize, val: T) {
        let (begin, end) = (new_nd.begin, new_nd.end);
        if begin == end {
            return;
        }

        new_nd.val = node.map_or(val, |x| self.combine(&x.val, &val));

        let mid = (begin + end) >> 1;

        if key <= mid {
            if let Some(x) = node { new_nd.set_right(x.right.clone()) }

            let mut left = Node::new(val, begin, mid);
            self.insert_node(&mut left, node.and_then(|x| x.left.as_ref()), key, val);

            new_nd.set_left(left.wrap());
        } else {
            if let Some(x) = node { new_nd.set_left(x.left.clone()) }

            let mut right = Node::new(val, mid + 1, end);
            self.insert_node(&mut right, node.and_then(|x| x.right.as_ref()), key, val);

            new_nd.set_right(right.wrap());
        }
    }

    /// **left** inclusive, **right** exclusive
    pub fn query(&self, left: usize, right: usize) -> Option<T> {
        self.roots
            .last()
            .and_then(|x| self.query_node(x, left, right - 1))
    }

    /// 主席树
    pub fn query_nth(&self, l: usize, r: usize, nth: usize) -> Option<T> {
        if r < l || nth <= 0 {
            panic!()
        }
        let l_node = if l > 0 { self.roots.get(l - 1) } else { None };
        let right = r.min(self.roots.len()).saturating_sub(1);
        let r_node = self.roots.get(right);

        let size1 = l_node.map_or(0, |x| x.size);
        let size2 = r_node.map_or(0, |x| x.size);

        if size2 - size1 < nth {
            None
        } else {
            self.query_nth_node(l_node, r_node, nth)
        }
    }

    fn query_nth_node(
        &self,
        node1: Option<&Rc<Node<T>>>,
        node2: Option<&Rc<Node<T>>>,
        nth: usize,
    ) -> Option<T> {
        if let Some(nd2) = node2 {
            let l_size1 = node1.map_or(0, |x| x.left.as_ref().map_or(0, |x| x.size));
            let l_size2 = nd2.left.as_ref().map_or(0, |x| x.size);

            // println!(
            //     "begin: {}, end: {}; node size: {}, {}",
            //     nd2.begin,
            //     nd2.end,
            //     node1.map_or(0, |x| x.size),
            //     nd2.size
            // );

            let d = l_size2 - l_size1;

            if nd2.begin == nd2.end {
                Some(nd2.val)
            } else if d >= nth {
                self.query_nth_node(node1.and_then(|x| x.left.as_ref()), nd2.left.as_ref(), nth)
            } else {
                self.query_nth_node(
                    node1.and_then(|x| x.right.as_ref()),
                    nd2.right.as_ref(),
                    nth - d,
                )
            }
        } else {
            None
        }
    }

    fn query_node(&self, node: &Rc<Node<T>>, left: usize, right: usize) -> Option<T> {
        if node.begin >= left && node.end <= right {
            return Some(node.val);
        }
        if node.begin > right || node.end < left {
            return None;
        }

        let res1 = if let Some(x) = &node.left {
            self.query_node(x, left, right)
        } else {
            None
        };

        let res2 = if let Some(x) = &node.right {
            self.query_node(x, left, right)
        } else {
            None
        };

        if let (Some(a), Some(b)) = (&res1, &res2) {
            Some(self.combine(a, b))
        } else {
            res1.or(res2)
        }
    }
}
