use super::binary_tree;
use ptr::NonNull;
use std::{cmp::Ordering, io::SeekFrom, iter::Map, marker::PhantomData, ops::Add, ptr};

type Edge<K, V> = Option<NonNull<Node<K, V>>>;

impl_debug!(TreapMap);

// todo: add size
#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    val: V,
    pri: f32,
    left: Edge<K, V>,
    right: Edge<K, V>,
}

pub struct TreapSet<T> {
    map: TreapMap<T, ()>,
}

pub struct TreapMap<K, V> {
    root: Edge<K, V>,
    _marker: PhantomData<Box<(K, V)>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            pri: rand::random(),
            left: None,
            right: None,
        }
    }

    fn wrap(self) -> NonNull<Self> {
        let b = Box::new(self);
        NonNull::from(Box::leak(b))
    }

    // fn iter(&self) -> tree::InorderIter<>
}

impl<T: Ord> TreapSet<T> {
    pub fn new() -> Self {
        Self {
            map: TreapMap::new(),
        }
    }

    pub fn from(v: Vec<T>) -> Self {
        Self {
            map: TreapMap::from(v.into_iter().map(|x| (x, ())).collect()),
        }
    }

    pub fn insert(&mut self, elem: T) {
        self.map.insert(elem, ());
    }

    pub fn get(&self, elem: &T) -> Option<&()> {
        self.map.get(elem)
    }

    pub fn iter(&self) -> binary_tree::Iter<Node<T, ()>> {
        (&self.map as &dyn binary_tree::BinaryTree<Node = Node<T, ()>>).iter()
    }

    pub fn into_iter(mut self) -> binary_tree::IntoIter<Node<T, ()>> {
        (&mut self.map as &mut dyn binary_tree::BinaryTree<Node = Node<T, ()>>).into_iter()
    }
}

impl<T: Add<Output = T> + Ord + Unit + Clone> TreapSet<T> {
    pub fn remove(&mut self, elem: &T) -> bool {
        self.map.remove(elem)
    }
}

impl<K, V> TreapMap<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> binary_tree::Iter<Node<K, V>> {
        (self as &dyn binary_tree::BinaryTree<Node = Node<K, V>>).iter()
    }

    pub fn into_iter(mut self) -> binary_tree::IntoIter<Node<K, V>> {
        (&mut self as &mut dyn binary_tree::BinaryTree<Node = Node<K, V>>).into_iter()
    }
}

impl<K: Ord, V> TreapMap<K, V> {
    pub fn from(v: Vec<(K, V)>) -> Self {
        if v.is_empty() {
            return Self::new();
        }

        let mut stk: Vec<NonNull<Node<K, V>>> = vec![];

        for (k, v) in v {
            let nd = Box::new(Node::new(k, v));
            let pri = nd.pri;
            let mut node = NonNull::from(Box::leak(nd));

            unsafe {
                while let Some(x) = stk.last() {
                    if x.as_ref().pri < pri {
                        node.as_mut().left = stk.pop();
                    } else {
                        break;
                    }
                }
                if let Some(x) = stk.last_mut() {
                    x.as_mut().right = Some(node);
                }
            }
            stk.push(node);
        }

        Self {
            root: stk.first().map(|x| *x),
            _marker: PhantomData
        }
    }

    fn split_node(mut node: Edge<K, V>, key: &K) -> (Edge<K, V>, Edge<K, V>) {
        if let Some(nd) = &mut node {
            let mut n = unsafe { nd.as_mut() };
            if &n.key < key {
                let (lt, ge) = Self::split_node(n.right.take(), &key);
                n.right = lt;
                (node, ge)
            } else {
                let (lt, ge) = Self::split_node(n.left.take(), &key);
                n.left = ge;
                (lt, node)
            }
        } else {
            (None, None)
        }
    }

    fn merge_node(mut node1: Edge<K, V>, mut node2: Edge<K, V>) -> Edge<K, V> {
        if let (Some(a), Some(b)) = (&mut node1, &mut node2) {
            unsafe {
                if a.as_ref().pri > b.as_ref().pri {
                    let mut bow_a = a.as_mut();
                    bow_a.right = Self::merge_node(bow_a.right.take(), node2);
                    node1
                } else {
                    let mut bow_b = b.as_mut();
                    bow_b.left = Self::merge_node(node1, bow_b.left.take());
                    node2
                }
            }
        } else {
            node1.or(node2)
        }
    }

    pub fn get<'a, 'b>(&'a self, key: &'b K) -> Option<&'a V> {
        let mut p = self.root.as_ref();
        while let Some(r) = p {
            let b = unsafe { r.as_ref() };
            let cur = &b.key;
            match cur.cmp(key) {
                Ordering::Less => p = b.right.as_ref(),
                Ordering::Greater => p = b.left.as_ref(),
                Ordering::Equal => return Some(&b.val),
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        if self.get(&key).is_some() {
            return false;
        }
        let (mut lt, ge) = Self::split_node(self.root.take(), &key);
        lt = Self::merge_node(lt, Some(Node::new(key, val).wrap()));
        self.root = Self::merge_node(lt, ge);
        true
    }
}

impl<K, V> Drop for TreapMap<K, V> {
    fn drop(&mut self) {
        let mut s = vec![];
        self.root.map(|x| s.push(x));

        while let Some(x) = s.pop() {
            let node = unsafe { Box::from_raw(x.as_ptr()) };
            node.left.map(|u| s.push(u));
            node.right.map(|u| s.push(u));
        }
    }
}

pub trait Unit {
    fn unit() -> Self;
}

impl<K: Add<Output = K> + Ord + Unit + Clone, V> TreapMap<K, V> {
    pub fn remove(&mut self, key: &K) -> bool {
        if self.get(&key).is_none() {
            return false;
        }
        let (l, r) = Self::split_node(self.root.take(), key);
        let (_, r) = Self::split_node(r, &key.clone().add(K::unit()));
        self.root = Self::merge_node(l, r);
        true
    }
}

impl Unit for i32 {
    fn unit() -> Self {
        1
    }
}

impl<K, V> binary_tree::BinaryTreeNode for Node<K, V> {
    type Key = K;
    type Value = V;

    fn left(&self) -> &Option<NonNull<Self>> {
        &self.left
    }

    fn mut_left(&mut self) -> &mut Option<NonNull<Self>> {
        &mut self.left
    }
    fn right(&self) -> &Option<NonNull<Self>> {
        &self.right
    }

    fn mut_right(&mut self) -> &mut Option<NonNull<Self>> {
        &mut self.right
    }

    fn move_kv(self) -> (Self::Key, Self::Value) {
        (self.key, self.val)
    }

    fn kv(&self) -> (&Self::Key, &Self::Value) {
        (&self.key, &self.val)
    }
}

impl<K, V> binary_tree::BinaryTree for TreapMap<K, V> {
    type Node = Node<K, V>;
    fn root(&self) -> Option<NonNull<Self::Node>> {
        self.root
    }
    fn mut_root(&mut self) -> &mut Option<NonNull<Self::Node>> {
        &mut self.root
    }
}
