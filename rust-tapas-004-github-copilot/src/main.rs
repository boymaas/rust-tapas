// implement a Tree using unsafe pointers

use std::ptr;

struct Node {
    value: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

struct Tree {
    root: *mut Node,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            root: ptr::null_mut(),
        }
    }

    fn insert(&mut self, value: i32) {
        let mut current = self.root;
        let mut parent = ptr::null_mut();

        while !current.is_null() {
            parent = current;
            if value < unsafe { (*current).value } {
                current = unsafe { (*current).left };
            } else {
                current = unsafe { (*current).right };
            }
        }

        let node = Box::new(Node::new(value));

        if parent.is_null() {
            self.root = Box::into_raw(node);
        } else if value < unsafe { (*parent).value } {
            unsafe { (*parent).left = Box::into_raw(node) };
        } else {
            unsafe { (*parent).right = Box::into_raw(node) };
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        fn drop_node(node: *mut Node) {
            if !node.is_null() {
                println!("Dropping node with value {}", unsafe { (*node).value });
                drop_node(unsafe { (*node).left });
                drop_node(unsafe { (*node).right });
                drop(unsafe { Box::from_raw(node) });
            }
        }

        drop_node(self.root);
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);
}
