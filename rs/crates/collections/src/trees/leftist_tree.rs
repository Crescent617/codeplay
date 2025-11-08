use std::{mem, ptr, rc::Rc};

#[derive(Debug)]
struct Node<T: Ord> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    dist: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LeftistTree<T: Ord> {
    root: Link<T>,
    size: usize,
}

impl<T: Ord> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            dist: 0,
        }
    }
}

impl<T: Ord> Default for LeftistTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> LeftistTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.elem)
    }

    pub fn merge(&mut self, other: Self) {
        self.size += other.size;
        self.root = Self::merge_node(self.root.take(), other.root);
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn get_node_dist(node: &Link<T>) -> usize {
        node.as_ref().map_or(0, |x| x.dist)
    }

    fn merge_node(a: Link<T>, b: Link<T>) -> Link<T> {
        if a.is_none() || b.is_none() {
            a.or(b)
        } else if let (Some(mut a), Some(mut b)) = (a, b) {
            if a.elem < b.elem {
                mem::swap(&mut a, &mut b);
            }

            a.right = Self::merge_node(a.right, Some(b));

            let ld = Self::get_node_dist(&a.left);
            let rd = Self::get_node_dist(&a.right);

            if rd > ld {
                mem::swap(&mut a.left, &mut a.right);
            }

            a.dist = rd.min(ld) + 1;
            Some(a)
        } else {
            panic!()
        }
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        let node = Some(Box::new(Node::new(val)));
        self.root = Self::merge_node(self.root.take(), node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.root.take().map(|x| {
            self.root = Self::merge_node(x.left, x.right);
            self.size -= 1;
            x.elem
        })
    }
}

/// Persistent LeftistTree
#[derive(Debug)]
pub struct PstLeftistTree<T: Clone> {
    roots: Vec<Rc<PstNode<T>>>,
}

#[derive(Debug, Clone)]
struct PstNode<T> {
    elem: T,
    left: PstLink<T>,
    right: PstLink<T>,
    dist: usize,
}

type PstLink<T> = Option<Rc<PstNode<T>>>;

impl<T> PstNode<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            dist: 0,
        }
    }
}

impl<T: Ord + Clone> Default for PstLeftistTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> PstLeftistTree<T> {
    pub fn new() -> Self {
        Self { roots: vec![] }
    }

    fn get_node_dist(node: &PstLink<T>) -> usize {
        node.as_ref().map_or(0, |x| x.dist)
    }

    pub fn push(&mut self, val: T) {
        let pre_root = self.roots.last().map(|x| x.to_owned());
        let new_node = Some(Rc::new(PstNode::new(val)));
        self.roots
            .push(Self::merge_node(pre_root, new_node).unwrap());
    }

    fn merge_node(a: PstLink<T>, b: PstLink<T>) -> PstLink<T> {
        if a.is_none() || b.is_none() {
            a.or(b)
        } else if let (Some(mut a), Some(mut b)) = (a, b) {
            if a.elem < b.elem {
                mem::swap(&mut a, &mut b);
            }

            let mut node = PstNode::clone(&a);

            node.right = Self::merge_node(node.right, Some(b));

            let ld = Self::get_node_dist(&node.left);
            let rd = Self::get_node_dist(&node.right);

            if rd > ld {
                mem::swap(&mut node.left, &mut node.right);
            }

            node.dist = rd.min(ld) + 1;
            Some(Rc::new(node))
        } else {
            panic!()
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let pre_root = self.roots.last().map(|x| PstNode::clone(x));
        pre_root.map(|x| {
            if let Some(root) = Self::merge_node(x.left, x.right) {
                self.roots.push(root);
            }
            x.elem
        })
    }
}
