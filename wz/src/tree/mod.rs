mod arena;
mod error;
mod id;
mod node;
mod traverse;

pub use crate::tree::{
    arena::Arena,
    error::{NodeError, NodeResult},
    id::NodeId,
    node::Node,
    traverse::{Ancestors, Traverse, TraverseType},
};

#[cfg(test)]
mod tests {

    use crate::tree::Arena;

    #[test]
    fn find_node() {
        let mut arena = Arena::new();
        let data1_ind = arena.new_node(1);
        let data2_ind = arena.new_node(2);
        let data3_ind = arena.new_node(3);
        assert_eq!(3, arena.count());
        let data1 = arena.get(data1_ind).unwrap();
        assert_eq!(1, *arena.get(data1_ind).unwrap().get().unwrap());
        let data2 = arena.get(data2_ind).unwrap();
        assert_eq!(2, *arena.get(data2_ind).unwrap().get().unwrap());
        let data3 = arena.get(data3_ind).unwrap();
        assert_eq!(3, *arena.get(data3_ind).unwrap().get().unwrap());
        assert_eq!(data1_ind, arena.get_node_id(data1).unwrap());
        assert_eq!(data2_ind, arena.get_node_id(data2).unwrap());
        assert_eq!(data3_ind, arena.get_node_id(data3).unwrap());
    }

    #[test]
    fn find_children() {
        let mut arena = Arena::new();
        let data1_ind = arena.new_node(1);
        let data2_ind = arena.new_node(2);
        let data3_ind = arena.new_node(3);
        data1_ind
            .insert(String::from("two"), data2_ind, &mut arena)
            .unwrap();
        data2_ind
            .insert(String::from("three"), data3_ind, &mut arena)
            .unwrap();
        let child_keys: Vec<&String> = data1_ind.children(&arena).collect();
        assert_eq!(1, child_keys.len());
        assert_eq!("two", child_keys[0]);
        let child_ind = data1_ind.get_child("two", &arena).unwrap();
        let child = arena.get(child_ind).unwrap();
        assert_eq!(2, *child.get().unwrap());
        let child_keys: Vec<&String> = data2_ind.children(&arena).collect();
        assert_eq!(1, child_keys.len());
        assert_eq!("three", child_keys[0]);
        let child_ind = data2_ind.get_child("three", &arena).unwrap();
        let child = arena.get(child_ind).unwrap();
        assert_eq!(3, *child.get().unwrap());
    }
}
