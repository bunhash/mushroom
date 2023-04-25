#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod arena;
mod id;
mod node;
mod traverse;

pub mod error;

pub use crate::{
    arena::Arena,
    id::NodeId,
    node::Node,
    traverse::{Ancestors, Traverse, TraverseType, TraverseUri},
};

#[cfg(test)]
mod tests {

    use crate::{Arena, NodeId};

    fn build_tree() -> (Arena<u32>, NodeId) {
        let mut arena = Arena::new();
        let data1_ind = arena.new_node(String::from("one"), 1);
        let data2_ind = arena.new_node(String::from("two"), 2);
        let data3_ind = arena.new_node(String::from("three"), 3);
        let data4_ind = arena.new_node(String::from("four"), 4);
        let data5_ind = arena.new_node(String::from("five"), 5);
        data1_ind.insert(data2_ind, &mut arena).unwrap();
        data2_ind.insert(data3_ind, &mut arena).unwrap();
        data2_ind.insert(data4_ind, &mut arena).unwrap();
        data4_ind.insert(data5_ind, &mut arena).unwrap();
        (arena, data1_ind)
    }

    #[test]
    fn find_node() {
        let (arena, root) = build_tree();
        assert_eq!(5, arena.count());
        assert!(root.has_child("two", &arena));
        let two_ind = root.get_child("two", &arena).unwrap();
        let two = arena.get(two_ind).unwrap();
        assert_eq!(2, *two.get().unwrap());
        assert_eq!(two_ind, arena.get_node_id(two).unwrap());
        assert_eq!(None, root.get_child("fail", &arena));
    }

    #[test]
    fn find_children() {
        let (arena, root) = build_tree();
        assert_eq!(5, arena.count());
        let child_keys: Vec<&str> = root.children(&arena).map(|s| s.as_str()).collect();
        assert_eq!(1, child_keys.len());
        assert_eq!("two", child_keys[0]);
        let child_ind = root.get_child("two", &arena).unwrap();
        let child = arena.get(child_ind).unwrap();
        assert_eq!(2, *child.get().unwrap());
        let mut child_keys: Vec<&str> = child_ind.children(&arena).map(|s| s.as_str()).collect();
        child_keys.sort();
        assert_eq!(2, child_keys.len());
        assert_eq!(["four", "three"], child_keys[..]);
        let child_ind = child_ind.get_child("three", &arena).unwrap();
        let child = arena.get(child_ind).unwrap();
        assert_eq!(3, *child.get().unwrap());
    }

    #[test]
    fn detach_child() {
        let (mut arena, root) = build_tree();
        assert_eq!(5, arena.count());
        assert!(root.has_child("two", &arena));
        let two_ind = root.get_child("two", &arena).unwrap();
        let child_keys: Vec<&str> = two_ind.children(&arena).map(|s| s.as_str()).collect();
        assert_eq!(2, child_keys.len());
        let four_ind = two_ind.get_child("four", &arena).unwrap();
        four_ind.detach(&mut arena);
        let child_keys: Vec<&str> = two_ind.children(&arena).map(|s| s.as_str()).collect();
        assert_eq!(1, child_keys.len());
    }

    #[test]
    fn remove_child() {
        let (mut arena, root) = build_tree();
        assert_eq!(5, arena.count());
        assert!(root.has_child("two", &arena));
        let two_ind = root.get_child("two", &arena).unwrap();
        let child_keys: Vec<&str> = two_ind.children(&arena).map(|s| s.as_str()).collect();
        assert_eq!(2, child_keys.len());
        let four_ind = two_ind.get_child("four", &arena).unwrap();

        // Remove 2 children (count = 5, in-use = 3)
        four_ind.remove(&mut arena);
        assert_eq!(5, arena.count());
        assert_eq!(3, root.descendents(&arena).count());
        let child_keys: Vec<&str> = two_ind.children(&arena).map(|s| s.as_str()).collect();
        assert_eq!(1, child_keys.len());

        // Add new children until the count needs to grow
        // count = 5, in-use = 4
        let new_node = arena.new_node(String::from("six"), 6);
        assert_eq!(5, arena.count());
        // count = 5, in-use = 5
        let _ = arena.new_node(String::from("seven"), 7);
        assert_eq!(5, arena.count());
        // count = 6, in-use = 6
        let _ = arena.new_node(String::from("eight"), 8);
        assert_eq!(6, arena.count());

        // Insert "six" into "two"
        two_ind.insert(new_node, &mut arena).unwrap();
        assert_eq!(4, root.descendents(&arena).count());
        let mut child_keys: Vec<&str> = two_ind.children(&arena).map(|s| s.as_str()).collect();
        child_keys.sort();
        assert_eq!(2, child_keys.len());
        assert_eq!(["six", "three"], child_keys[..]);
    }
}
