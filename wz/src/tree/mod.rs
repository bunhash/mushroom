pub mod node;
mod tree;

pub use node::NodeRef;
pub use tree::Tree;

#[cfg(test)]
mod tests {

    use crate::tree::Tree;

    #[test]
    fn test1() {
        let mut tree = Tree::with_root(String::from("one"));
        let node_ref = tree.root().unwrap();
        tree.add_child(node_ref, String::from("two"));
        
        let mut results: Vec<String> = Vec::new();
        tree.traverse(&mut |s| results.push(s.clone()));

        assert_eq!(&results[..], ["one", "two"]);
    }
}
