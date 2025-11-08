#![allow(dead_code)]

use super::binary_tree;
use ptr::NonNull;
use std::{cmp::Ordering, fmt, marker::PhantomData, mem, ops::Add, ptr, todo};

type Link<K, V> = Option<NonNull<Node<K, V>>>;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

use Color::*;

use super::treap::TreapMap;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    color: Color,
    size: usize,
}

impl_debug!(RBTreeMap);

pub struct RBTreeMap<K, V> {
    root: Link<K, V>,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            left: None,
            right: None,
            color: Red,
            size: 1,
        }
    }

    fn wrap(self) -> NonNull<Self> {
        let b = Box::new(self);
        NonNull::from(Box::leak(b))
    }

    fn set_red(&mut self) {
        self.set_color(Red);
    }

    fn set_black(&mut self) {
        self.set_color(Black);
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
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

    fn replace_left(&mut self, pointer: NonNull<Self>) -> Link<K, V> {
        let pre = self.take_left();
        self.left.replace(pointer);
        self.size += unsafe { pointer.as_ref().size };
        pre
    }

    fn replace_right(&mut self, pointer: NonNull<Self>) -> Link<K, V> {
        let pre = self.take_right();
        self.right.replace(pointer);
        self.size += unsafe { pointer.as_ref().size };
        pre
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

    fn is_red(&self) -> bool {
        match self.color {
            Red => true,
            _ => false,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn get_successor(&self) -> Option<NonNull<Self>> {
        let mut cur = self.right;
        let mut pre = None;

        while let Some(p) = cur {
            pre = cur;
            cur = unsafe { p.as_ref().left };
        }
        pre
    }

    fn flip_color(&mut self) {
        let new_color = match self.color {
            Red => Black,
            Black => Red,
        };
        self.color = new_color;
    }

    unsafe fn flip_colors(&mut self) { unsafe {
        let mut left = self.left.expect("left must exist");
        let mut right = self.right.expect("left must exist");
        debug_assert_eq!(left.as_ref().color, right.as_ref().color);
        debug_assert_ne!(left.as_ref().color, self.color);

        self.flip_color();
        right.as_mut().flip_color();
        left.as_mut().flip_color();
    }}

    unsafe fn left_rotate(&mut self) -> Link<K, V> { unsafe {
        // left rotate a red link
        //          <1>                   <2>
        //        /    \\               //    \
        //       *      <2>    ==>     <1>     *
        //             /   \          /   \
        //            *     *        *     *
        self.take_right().map(|mut par| {
            mem::swap(&mut self.color, &mut par.as_mut().color);
            self.set_right(par.as_mut().take_left());
            par.as_mut()
                .replace_left(NonNull::new_unchecked(self as *mut Self));
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
            mem::swap(&mut self.color, &mut par.as_mut().color);
            self.set_left(par.as_mut().take_right());
            par.as_mut()
                .replace_right(NonNull::new_unchecked(self as *mut Self));
            par
        })
    }}
}

impl<K: Ord, V> Default for RBTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> RBTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.root.map_or(0, |x| unsafe { x.as_ref().size })
    }

    pub fn insert(&mut self, key: K, val: V) {
        unsafe {
            self.root.replace(Self::insert_node(self.root, key, val));
            if let Some(mut x) = self.root { x.as_mut().set_black() }
        }
    }

    unsafe fn insert_node(node: Link<K, V>, key: K, val: V) -> NonNull<Node<K, V>> { unsafe {
        if let Some(mut nd) = node {
            let p = nd.as_mut();
            match p.key.cmp(&key) {
                Ordering::Equal => p.val = val,
                Ordering::Greater => {
                    p.replace_left(Self::insert_node(p.left, key, val));
                }
                Ordering::Less => {
                    p.replace_right(Self::insert_node(p.right, key, val));
                }
            }
            Self::fixup(nd)
        } else {
            Node::new(key, val).wrap()
        }
    }}

    unsafe fn fixup(mut node: NonNull<Node<K, V>>) -> NonNull<Node<K, V>> { unsafe {
        if !Self::is_some_red(node.as_ref().left) && Self::is_some_red(node.as_ref().right) {
            node = node.as_mut().left_rotate().expect("must exist");
        }

        let left = node.as_ref().left;
        if Self::is_some_red(left) && left.is_some_and(|l| Self::is_some_red(l.as_ref().left)) {
            node = node.as_mut().right_rotate().expect("must exist");
        }

        if Self::is_some_red(node.as_ref().left) && Self::is_some_red(node.as_ref().right) {
            debug_assert!(!node.as_ref().is_red());
            node.as_mut().flip_colors();
        }
        node.as_mut().refresh_size();
        node
    }}

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur = self.root;
        while let Some(p) = cur {
            unsafe {
                match p.as_ref().key.cmp(key) {
                    Ordering::Equal => return Some(&(*p.as_ptr()).val),
                    Ordering::Greater => cur = p.as_ref().left,
                    Ordering::Less => cur = p.as_ref().right,
                }
            }
        }
        None
    }

    fn is_some_red(node: Link<K, V>) -> bool {
        node.is_some_and(|x| unsafe { x.as_ref().is_red() })
    }

    pub fn remove(&mut self, key: &K) -> bool {
        if self.get(key).is_none() {
            false
        } else {
            unsafe {
                if let Some(mut root) = self.root {
                    let r = root.as_ref();
                    if !Self::is_some_red(r.left) && !Self::is_some_red(r.right) {
                        root.as_mut().set_red();
                    }
                    self.root = Self::_remove(root, key);
                    if let Some(mut x) = self.root { x.as_mut().set_black() }
                }
            }
            true
        }
    }

    unsafe fn move_red_left(mut node: NonNull<Node<K, V>>) -> NonNull<Node<K, V>> { unsafe {
        node.as_mut().flip_colors();
        let mut r = node.as_ref().right.expect("must exist");

        if Self::is_some_red(r.as_ref().left) {
            node.as_mut().right = r.as_mut().right_rotate();
            node = node.as_mut().left_rotate().expect("must exist");
            node.as_mut().flip_colors();
        }
        node
    }}

    unsafe fn move_red_right(mut node: NonNull<Node<K, V>>) -> NonNull<Node<K, V>> { unsafe {
        node.as_mut().flip_colors();
        let l = node.as_ref().left.expect("must exist");

        if Self::is_some_red(l.as_ref().left) {
            node = node.as_mut().right_rotate().expect("must exist");
            node.as_mut().flip_colors();
        }
        node
    }}

    unsafe fn delete_node(node: NonNull<Node<K, V>>) { unsafe {
        let b = Box::from_raw(node.as_ptr());
        debug_assert!(b.is_red());
    }}

    unsafe fn _remove(mut node: NonNull<Node<K, V>>, key: &K) -> Link<K, V> { unsafe {
        if key < &node.as_ref().key {
            let l = node.as_ref().left.expect("must exist");

            if !l.as_ref().is_red() && !Self::is_some_red(l.as_ref().left) {
                node = Self::move_red_left(node);
            }
            let p = node.as_mut();
            p.left = Self::_remove(p.left.expect("must exist"), key);
        } else {
            if &node.as_ref().key == key && node.as_ref().is_leaf() {
                Self::delete_node(node);
                return None;
            }
            // 调用move_red_right必须保证两个儿子都是black
            if Self::is_some_red(node.as_ref().left) {
                // 这里没有直接取右儿子的值，主要是子树根节点node需要在返回时fixup
                node = node.as_mut().right_rotate().expect("must exist");
            }

            let r = node.as_ref().right.expect("must exist");
            if !r.as_ref().is_red() && !Self::is_some_red(r.as_ref().left) {
                node = Self::move_red_right(node);
            }

            if &node.as_ref().key == key {
                if let Some(mut suc) = node.as_ref().get_successor() {
                    mem::swap(&mut node.as_mut().key, &mut suc.as_mut().key);
                    mem::swap(&mut node.as_mut().val, &mut suc.as_mut().val);
                } else {
                    panic!()
                }
            }
            let p = node.as_mut();
            p.right = Self::_remove(p.right.expect("must exist"), key);
        }
        Some(Self::fixup(node))
    }}
}

impl<K, V> Drop for RBTreeMap<K, V> {
    fn drop(&mut self) {
        let mut s = vec![];
        if let Some(x) = self.root { s.push(x) }

        while let Some(x) = s.pop() {
            let node = unsafe { Box::from_raw(x.as_ptr()) };
            if let Some(u) = node.left { s.push(u) }
            node.right.map(|u| unsafe {
                debug_assert!(!u.as_ref().is_red());
                s.push(u);
            });
        }
    }
}
