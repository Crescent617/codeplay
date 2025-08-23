#![allow(unused_imports)]

pub mod binary_tree;
pub mod bitree;
pub mod dsu;
pub mod leftist_tree;
pub mod rbtree;
pub mod segment_tree;
pub mod skip_list;
pub mod sparse_table;
pub mod splay_tree;
pub mod stream;
pub mod treap;
pub use bitree::BIT;
pub use leftist_tree::LeftistTree;
pub use rbtree::RBTreeMap;
pub use segment_tree::{PstSegTree, SegTree};
pub use skip_list::SkipListSet;
pub use splay_tree::SplayTreeMap;
pub use treap::TreapMap;

#[allow(unused_macros)]
macro_rules! timeit {
    ($block: tt) => {{
        use std::time;
        let t = time::Instant::now();
        $block;
        let d = t.elapsed();
        println!("Time usage: {:?}", d);
        d
    }};

    ($name: expr, $block: tt) => {{
        use std::time;
        let t = time::Instant::now();
        $block;
        let d = t.elapsed();
        println!("{} use: {:?}", $name, d);
        d
    }};
}

#[allow(unused_macros)]
macro_rules! test_map {
    ($t: tt) => {{
        let mut s = $t::new();
        let mut b = BTreeMap::new();

        for i in 0..2000 {
            let k = rand::random::<i32>();
            b.insert(k, i);
            s.insert(k, i);
            assert_eq!(b.len(), s.len());
        }

        for _ in 0..2000 {
            let k = &rand::random::<i32>();
            assert_eq!(b.get(k), s.get(k));
        }

        for _ in 0..1000 {
            let k = rand::random::<i32>();
            assert_eq!(s.remove(&k), b.remove(&k).is_some());
            assert_eq!(s.len(), b.len());
        }
    }};
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use skip_list::SkipListMap;
    use std::{
        collections::{BTreeMap, BTreeSet, BinaryHeap},
        time,
    };
    use treap::TreapSet;

    #[test]
    fn test_leftist_tree() {
        let n = 10000;
        let mut heap = LeftistTree::new();
        let mut rng = rand::thread_rng();

        let t = time::Instant::now();
        for _ in 0..n {
            heap.push(rng.gen_range(i32::MIN..i32::MAX));
        }
        for _ in 0..n {
            heap.pop();
        }
        println!("LTree use: {:?}", t.elapsed());

        let mut heap = BinaryHeap::new();

        let t = time::Instant::now();
        for _ in (0..n).rev() {
            heap.push(rng.gen_range(i32::MIN..i32::MAX));
        }
        for _ in 0..n {
            heap.pop();
        }
        println!("BHeap use: {:?}", t.elapsed());

        let mut h1 = LeftistTree::new();
        let mut h2 = BinaryHeap::new();

        for _ in 0..n {
            let r = rng.gen_range(i32::MIN..i32::MAX);
            h1.push(r);
            h2.push(r);
        }
        for _ in 0..n {
            assert_eq!(h1.pop(), h2.pop());
            assert_eq!(h1.len(), h2.len());
        }
    }

    #[test]
    fn test_segtree() {
        let mut seg = PstSegTree::new(0, 1000, |&x, &y| x + y);
        let v = vec![10, 20, 30, 50, 100];
        for x in v.into_iter().enumerate() {
            seg.insert(x.0, x.1);
        }
        assert_eq!(seg.query_nth(0, 10, 1), Some(10));
        assert_eq!(seg.query_nth(0, 10, 5), Some(100));
        assert_eq!(seg.query_nth(0, 10, 100), None);

        let v: Vec<_> = (0..100).collect();
        let mut seg = SegTree::from(v, Box::new(|x: &i32, y: &i32| *x.max(y)));

        seg.insert(50, 1);
        assert_eq!(seg.query(45, 50), Some(49));
        assert_eq!(seg.query(0, 200), None);
    }

    #[test]
    fn test_bit() {
        let mut b = BIT::new(10);
        b.update(1, 10);
        b.update(2, 1);
        b.update(3, 15);
        assert_eq!(b.query(2, 3), 1);
        b.update(6, 2);
        assert_eq!(b.query(3, 7), 17);
    }

    #[test]
    fn test_treap() {
        let mut v: Vec<_> = (0..10).zip(0..10).collect();
        let mut t = TreapMap::new();

        for (k, v) in v.iter().rev() {
            t.insert(*k, *v);
        }
        assert!(t.remove(&0));
        assert!(t.remove(&5));

        v.remove(5);
        v.remove(0);

        for (i, x) in t.iter().enumerate() {
            // println!("{}", x);
            assert_eq!((*x.0, *x.1), v[i]);
        }
        assert_eq!(v, t.into_iter().collect::<Vec<_>>());

        let mut rng = rand::thread_rng();
        let n = 10000;
        let mut tr = TreapSet::new();
        let mut b = BTreeSet::new();

        for _ in 0..n {
            let r = rng.gen_range(i32::MIN..i32::MAX);
            tr.insert(r);
            b.insert(r);
        }

        for _ in 0..n {
            let r = &rng.gen_range(i32::MIN..i32::MAX);
            assert_eq!(b.remove(&r), tr.remove(&r));
            assert_eq!(b.remove(&r), tr.remove(&r));
        }
    }

    #[test]
    fn test_skip_list() {
        test_map!(SkipListMap);
        let mut rng = rand::thread_rng();

        let n = 100000;

        let mut skip = SkipListSet::new();
        timeit!("SkipList insert", {
            for _ in 0..n {
                skip.insert(rng.gen_range(i32::MIN..i32::MAX));
            }
        });
        timeit!("SkipList get", {
            for _ in 0..n {
                skip.get(&rng.gen_range(i32::MIN..i32::MAX));
            }
        });
        timeit!("SkipList remove", {
            for _ in 0..n {
                skip.remove(&rng.gen_range(i32::MIN..i32::MAX));
            }
        });
        println!("SkipList height: {}", skip.height());
        println!("SkipList: {:?}", skip);

        let mut btree = BTreeSet::new();
        timeit!("BTree insert", {
            for _ in 0..n {
                btree.insert(rng.gen_range(i32::MIN..i32::MAX));
            }
        });
        timeit!("BTree get", {
            for _ in 0..n {
                btree.get(&rng.gen_range(i32::MIN..i32::MAX));
            }
        });
        timeit!("BTree remove", {
            for _ in 0..n {
                btree.remove(&rng.gen_range(i32::MIN..i32::MAX));
            }
        });
    }

    #[test]
    fn test_rbtree() {
        test_map!(RBTreeMap);

        let n = 400000;
        timeit!("RBTree", {
            let mut t = RBTreeMap::new();
            for _ in 0..n {
                t.insert(rand::random::<i32>(), ());
            }
            for _ in 0..n {
                t.remove(&rand::random::<i32>());
            }
        });

        timeit!("BTree", {
            let mut t = BTreeMap::new();
            for _ in 0..n {
                t.insert(rand::random::<i32>(), ());
            }
            for _ in 0..n {
                t.remove(&rand::random::<i32>());
            }
        });
    }

    #[test]
    fn test_splay() {
        test_map!(SplayTreeMap);

        let n = 400000;
        timeit!("SplayTree", {
            let mut t = RBTreeMap::new();
            for _ in 0..n {
                t.insert(rand::random::<i32>(), ());
            }
            for _ in 0..n {
                t.remove(&rand::random::<i32>());
            }
        });

        timeit!("BTree", {
            let mut t = BTreeMap::new();
            for _ in 0..n {
                t.insert(rand::random::<i32>(), ());
            }
            for _ in 0..n {
                t.remove(&rand::random::<i32>());
            }
        });
    }

    #[test]
    fn test_st() {
        let st = sparse_table::SparseTable::new(&[1, 0, 3, 0, 5, 0]);
        assert_eq!(st.query(0, 6), 0);
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(2, 4), 0);
        assert_eq!(st.query(5, 6), 0);
    }
}
