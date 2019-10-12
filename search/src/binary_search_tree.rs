type ChildLink<i32> = Option<Box<Node<i32>>>;

#[derive(Debug)]
struct Node<i32> {
    value: i32,
    left: ChildLink<i32>,
    right: ChildLink<i32>,
}

pub struct Tree<i32> {
    root: ChildLink<i32>,
}

pub trait BSTNode<I32> {
    fn new() -> Self;
    fn insert(&self, value: i32);
    fn find(&self, value: i32);
    fn minimum(&self, k: i32); // k-s minimum
    fn pass(&self);
    fn left_rotation(&self);
    fn right_rotation(&self);
    fn plain_root(&self); // put in the root
    fn balance(&self);
}

impl Tree<i32> {
    fn new() -> Self {
        Tree { root: None }
    } 
}
