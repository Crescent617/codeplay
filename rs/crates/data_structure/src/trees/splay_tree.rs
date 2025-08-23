#![allow(dead_code)]

use ptr::NonNull;
use std::{cmp::Ordering, marker::PhantomData, mem, ops::Add, ptr, todo};

use super::binary_tree;

type Link<K, V> = Option<NonNull<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    size: usize,
}

impl_debug!(SplayTreeMap);

pub struct SplayTreeMap<K, V> {
    root: Link<K, V>,
    _marker: PhantomData<(K, V)>
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            left: None,
            right: None,
            size: 1,
        }
    }

    fn wrap(self) -> NonNull<Self> {
        let b = Box::new(self);
        NonNull::from(Box::leak(b))
    }

    fn set_left(&mut self, node: Link<K, V>) {
        self.left = node;
        unsafe {
            self.refresh_size();
        }
    }

    fn set_right(&mut self, node: Link<K, V>) {
        self.right = node;
        unsafe {
            self.refresh_size();
        }
    }

    unsafe fn refresh_size(&mut self) { unsafe {
        let f = |u: Link<K, V>| u.map_or(0, |x| x.as_ref().size);
        self.size = 1 + f(self.left) + f(self.right);
    }}

    fn take_left(&mut self) -> Link<K, V> {
        self.left.map(|x| unsafe { self.size -= x.as_ref().size });
        self.left.take()
    }

    fn take_right(&mut self) -> Link<K, V> {
        self.right.map(|x| unsafe { self.size -= x.as_ref().size });
        self.right.take()
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    unsafe fn left_rotate(&mut self) -> Link<K, V> { unsafe {
        // left rotate a red link
        //          <1>                   <2>
        //        /    \\               //    \
        //       *      <2>    ==>     <1>     *
        //             /   \          /   \
        //            *     *        *     *
        self.take_right().map(|mut par| {
            self.set_right(par.as_mut().take_left());
            par.as_mut()
                .set_left(Some(NonNull::new_unchecked(self as *mut Self)));
            par
        })
    }}

    unsafe fn right_rotate(&mut self) -> Link<K, V> { unsafe {
        // right rotate a red link
        //            <1>               <2>
        //          //    \           /    \\
        //         <2>     *   ==>   *      <1>
        //        /   \                    /   \
        //       *     *                  *     *
        self.take_left().map(|mut par| {
            self.set_left(par.as_mut().take_right());
            par.as_mut()
                .set_right(Some(NonNull::new_unchecked(self as *mut Self)));
            par
        })
    }}
}

impl<K: Ord, V> Default for SplayTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> SplayTreeMap<K, V> {
    pub fn new() -> Self {
        Self { root: None, _marker: PhantomData }
    }

    pub fn len(&self) -> usize {
        self.root.map_or(0, |x| unsafe { x.as_ref().size })
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.root = unsafe { Self::insert_node(self.root, key, val) };
    }

    unsafe fn insert_node(node: Link<K, V>, key: K, val: V) -> Link<K, V> { unsafe {
        if let Some(mut nd) = node {
            match nd.as_ref().key.cmp(&key) {
                Ordering::Equal => {
                    nd.as_mut().val = val;
                    node
                }
                Ordering::Greater => {
                    let root = Self::insert_node(nd.as_mut().left, key, val);
                    nd.as_mut().set_left(root);
                    nd.as_mut().right_rotate()
                }
                Ordering::Less => {
                    let root = Self::insert_node(nd.as_mut().right, key, val);
                    nd.as_mut().set_right(root);
                    nd.as_mut().left_rotate()
                }
            }
        } else {
            Some(Node::new(key, val).wrap())
        }
    }}

    pub fn get(&mut self, key: &K) -> Option<&V> {
        unsafe {
            self.root = Self::splay(self.root, key);
            self.root
                .filter(|x| &x.as_ref().key == key)
                .map(|x| &(*x.as_ptr()).val)
        }
    }

    unsafe fn splay(node: Link<K, V>, key: &K) -> Link<K, V> { unsafe {
        if let Some(mut nd) = node {
            match nd.as_ref().key.cmp(key) {
                Ordering::Equal => node,
                Ordering::Greater => {
                    let root = Self::splay(nd.as_mut().left, key);
                    nd.as_mut().set_left(root);
                    nd.as_mut().right_rotate().or(node)
                }
                Ordering::Less => {
                    let root = Self::splay(nd.as_mut().right, key);
                    nd.as_mut().set_right(root);
                    nd.as_mut().left_rotate().or(node)
                }
            }
        } else {
            None
        }
    }}

    pub fn remove(&mut self, key: &K) -> bool {
        if self.get(key).is_none() {
            return false;
        }

        self.root.take().map(|x| unsafe {
            let mut root = Box::from_raw(x.as_ptr());
            let mut left = root.take_left();

            self.root = root.right;

            while let Some(mut l) = left {
                if l.as_ref().right.is_none() {
                    l.as_mut().set_right(root.right);
                    self.root.replace(l);
                } else {
                    left = l.as_mut().right_rotate();
                }
            }
        });
        true
    }
}

impl<K, V> Drop for SplayTreeMap<K, V> {
    fn drop(&mut self) {
        let mut s = vec![];
        if let Some(x) = self.root { s.push(x) }

        while let Some(x) = s.pop() {
            let node = unsafe { Box::from_raw(x.as_ptr()) };
            if let Some(u) = node.left { s.push(u) }
            if let Some(u) = node.right { s.push(u) }
        }
    }
}
