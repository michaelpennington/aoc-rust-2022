use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type SharedNode<T> = Rc<RefCell<Node<T>>>;
pub type WeakNode<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug, Default)]
pub struct Tree<T> {
    pub root: Option<SharedNode<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn set_root(&mut self, value: T) -> SharedNode<T> {
        let root = Node::new(value);
        self.root = Some(root.clone());
        root
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub parent: Option<WeakNode<T>>,
    pub children: Vec<SharedNode<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> SharedNode<T> {
        Rc::new(RefCell::new(Node {
            value,
            parent: None,
            children: vec![],
        }))
    }

    pub fn add_child(parent: &SharedNode<T>, value: T) -> SharedNode<T> {
        let child = Node::new(value);
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child.clone());
        child
    }

    pub fn print_tree(node: &SharedNode<T>, depth: usize)
    where
        T: std::fmt::Debug,
    {
        let n = node.borrow();

        println!("{}{:?}", "  ".repeat(depth), n.value);

        for child in &n.children {
            Node::print_tree(child, depth + 1);
        }
    }

    pub fn get_parent(node: &SharedNode<T>) -> Option<SharedNode<T>> {
        node.borrow()
            .parent
            .as_ref()
            .and_then(|weak_parent| weak_parent.upgrade())
    }

    pub fn iter_children(node: &SharedNode<T>) -> impl Iterator<Item = SharedNode<T>> {
        node.borrow().children.clone().into_iter()
    }
}
