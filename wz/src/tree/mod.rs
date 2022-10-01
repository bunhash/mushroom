mod tree;
mod node;

pub use tree::Tree;
pub use node::{Node, NodeRef};

#[cfg(test)]
mod tests {

    use crate::tree::Tree;

    #[test]
    fn basic_tree() {
        let mut tree = Tree::with_root(8);
        let _ = tree.new_node(10);
    }
}
