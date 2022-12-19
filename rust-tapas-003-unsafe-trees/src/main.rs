use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug)]
struct Tree {
    root: Option<*mut Node>,
}

impl Tree {
    fn add_node(&mut self, v: usize) {
        let node = Box::into_raw(Box::new(Node::new(v)));
        if let Some(root) = self.root {
            unsafe { (*root).children.push(node) };
        } else {
            self.root = Some(node);
            // create the root and add to children
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        if let Some(root) = self.root {
            // walk children first, make sure memory is freed.
            for child in unsafe { &(*root).children } {
                eprintln!("drop child");
                let node = unsafe { Box::from_raw(*child) };
                drop(node);
            }
            // free the root
            //
            eprintln!("drop root");
            let root = unsafe { Box::from_raw(root) };
            drop(root);
        }
    }
}

#[derive(Default, Debug)]
struct Node {
    parent: Option<*mut Node>,
    children: Vec<*mut Node>,
    data: usize,
}

impl Node {
    fn new(data: usize) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
}

fn main() {
    let mut tree = Tree::default();
    tree.add_node(5);
    tree.add_node(6);
    tree.add_node(7);
    tree.add_node(8);
    tree.add_node(9);

    dbg!(&tree);

    drop(tree);
}
