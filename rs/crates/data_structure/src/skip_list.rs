#![allow(dead_code)]

use fmt::Display;
use std::{marker::PhantomData, ptr::NonNull};
use std::{cmp::Ordering, fmt, fmt::Debug};
use std::{
    fmt::Formatter,
    mem::{size_of, size_of_val},
};

const P: f32 = 0.3;
const MAX_LEVEL: usize = 32;

type Link<K, V> = Option<NonNull<Node<K, V>>>;

pub struct SkipListSet<T> {
    map: SkipListMap<T, ()>,
}

impl<T: Ord> SkipListSet<T> {
    pub fn new() -> Self {
        Self {
            map: SkipListMap::new(),
        }
    }

    pub fn insert(&mut self, key: T) -> bool {
        self.map.insert(key, ())
    }

    pub fn get(&self, key: &T) -> Option<&()> {
        self.map.get(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn height(&self) -> usize {
        self.map._height
    }

    pub fn remove(&mut self, key: &T) -> bool {
        self.map.remove(key)
    }
}

pub struct SkipListMap<K, V> {
    heads: Vec<Link<K, V>>,
    _marker: PhantomData<(K, V)>,
    _len: usize,
    _height: usize,
}

struct Node<K, V> {
    nexts: Vec<Link<K, V>>,
    key: K,
    val: V,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            nexts: vec![None; Self::random_level()],
            key,
            val,
        }
    }

    fn random_level() -> usize {
        let mut l = 1;
        while rand::random::<f32>() < P && l < MAX_LEVEL {
            l += 1;
        }
        l
    }
}

impl<K: Ord, V> SkipListMap<K, V> {
    pub fn new() -> Self {
        Self {
            heads: vec![None; MAX_LEVEL],
            _len: 0,
            _height: 0,
            _marker: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        self._len
    }

    pub fn height(&self) -> usize {
        self._height
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        if let Some(mut nd) = Self::get_node(&self.heads, self._height, &key) {
            unsafe {
                nd.as_mut().val = val;
            }
            false
        } else {
            self._len += 1;
            let node = Box::new(Node::new(key, val));
            self._height = self._height.max(node.nexts.len());
            unsafe {
                Self::insert_node(
                    &mut self.heads,
                    self._height,
                    NonNull::new_unchecked(Box::into_raw(node)),
                );
            }
            true
        }
    }

    fn insert_node(mut nexts: &mut [Link<K, V>], mut h: usize, mut node: NonNull<Node<K, V>>) {
        let node_level = unsafe { node.as_ref().nexts.len() };

        while h > 0 {
            if let Some(p) = nexts[h - 1] {
                unsafe {
                    match node.as_ref().key.cmp(&p.as_ref().key) {
                        Ordering::Equal => panic!(),
                        Ordering::Less => {
                            if h <= node_level {
                                node.as_mut().nexts[h - 1].replace(p);
                                nexts[h - 1].replace(node);
                            }
                            h -= 1;
                        }
                        Ordering::Greater => nexts = &mut (*p.as_ptr()).nexts,
                    }
                }
            } else {
                if h <= node_level {
                    nexts[h - 1].replace(node);
                }
                h -= 1;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_node(&self.heads, self._height, key).map(|x| unsafe { &(*x.as_ptr()).val })
    }

    fn get_node(mut nexts: &[Link<K, V>], mut h: usize, key: &K) -> Option<NonNull<Node<K, V>>> {
        while h > 0 {
            if let Some(p) = nexts[h - 1] {
                match key.cmp(unsafe { &p.as_ref().key }) {
                    Ordering::Equal => return Some(p),
                    Ordering::Less => h -= 1,
                    Ordering::Greater => nexts = unsafe { &mut (*p.as_ptr()).nexts },
                }
            } else {
                h -= 1;
            }
        }
        None
    }

    fn remove_node(mut nexts: &mut [Link<K, V>], mut h: usize, key: &K) -> Option<Box<Node<K, V>>> {
        let mut to_remove = None;
        while h > 0 {
            if let Some(p) = nexts[h - 1] {
                match key.cmp(unsafe { &p.as_ref().key }) {
                    Ordering::Equal => unsafe {
                        to_remove.replace(p);
                        nexts[h - 1] = p.as_ref().nexts[h - 1];
                        h -= 1;
                    },
                    Ordering::Less => h -= 1,
                    Ordering::Greater => nexts = unsafe { &mut (*p.as_ptr()).nexts },
                }
            } else{
                h -= 1;
            }
        }
        to_remove.map(|x| unsafe { Box::from_raw(x.as_ptr()) })
    }

    pub fn remove(&mut self, key: &K) -> bool {
        if self.get(key).is_none() {
            false
        } else {
            let n = self._height;
            Self::remove_node(&mut self.heads, n, key);
            true
        }
    }
}

impl<K, V> Drop for SkipListMap<K, V> {
    fn drop(&mut self) {
        let mut node = self.heads[0];
        while let Some(p) = node {
            unsafe {
                let b = Box::from_raw(p.as_ptr());
                node = b.nexts[0];
            }
        }
    }
}

impl<K: Debug, V: Debug> Debug for SkipListMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in (0..self._height).rev() {
            let mut node = self.heads[i];
            let mut cnt = 0;

            while let Some(p) = node {
                cnt += 1;
                unsafe {
                    if cnt < 5 {
                        if size_of_val(&p.as_ref().val) > 0 {
                            write!(f, "({:?} => {:?}) -> ", p.as_ref().key, p.as_ref().val)?;
                        } else {
                            write!(f, "{:?} -> ", p.as_ref().key)?;
                        }
                    }
                    node = p.as_ref().nexts[i];
                }
            }

            write!(f, "... end, total {} elements.\n", cnt)?;
        }
        write!(f, "")
    }
}

impl<T: Debug> fmt::Debug for SkipListSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

pub struct Iter<'a, K, V> {
    pointer: Link<K, V>,
    _marker: PhantomData<&'a Node<K, V>>,
}

pub struct IntoIter<K, V> {
    pointer: Link<K, V>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.map(|x| unsafe {
            let b = Box::from_raw(x.as_ptr());
            let output = (b.key, b.val);
            self.pointer = b.nexts[0];
            output
        })
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.pointer.map(|x| unsafe {
            let b = x.as_ptr();
            let output = (&(*b).key, &(*b).val);
            self.pointer = (*b).nexts[0];
            output
        })
    }
}


impl<K, V> IntoIterator for SkipListMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter {
            pointer: self.heads[0].take(),
        }
    }
}

