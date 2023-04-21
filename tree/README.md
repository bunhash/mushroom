Named Tree (Directory) Structure
================================

Defines a directory-like structure where each Node has a name. The structure
adheres to an arena structure format so there are no uses of Box or Rc.

```rust
use tree::Arena;

let mut arena = Arena::new();
let root = arena.new_node(String::from("root"), 1);
let child1 = arena.new_node(String::from("child1"), 4);
root.insert(child1, &mut arena);
let child2 = arena.new_node(String::from("child2"), 3);
root.insert(child2, &mut arena);
let descendent1 = arena.new_node(String::from("descendent1"), 8);
child1.insert(descendent1, &mut arena);

assert_eq!(child1, root.get_child("child1", &arena).unwrap());
assert_eq!(descendent1, child1.get_child("descendent1", &arena).unwrap());
assert_eq!(8, *arena.get(descendent1).unwrap().get().unwrap());
```
