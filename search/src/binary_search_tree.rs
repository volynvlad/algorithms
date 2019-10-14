use std::fmt;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Greater, Equal};


type NodeWrapper = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    left: NodeWrapper,
    right: NodeWrapper,
} 

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree {
    pub root: NodeWrapper,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree {
            root: None,
        }
    }

    fn display_helper(node: &Node) {
        match node.left {
            Some(ref n) => {
                BinaryTree::display_helper(n);
                println!("{}", n.value);
            },
            None => {
                println!("None");
            }
        }
        match node.right {
            Some(ref n) => {
                BinaryTree::display_helper(n);
                println!("{}", n.value);
            },
            None => { 
                println!("None");
            }
        }
    }

    pub fn display(&self) {
        match self.root {
            Some(ref n) => {
                println!("{}", n.value);
                BinaryTree::display_helper(n);
            },
            None => {
                println!("Empty tree.");
            }
        }
    }

    fn insert_helper(node: &mut Node, value: i32) {
        if value < node.value {
            match node.left {
                Some(ref mut n) => {
                    BinaryTree::insert_helper(n, value);
                },
                None => {
                    node.left = Some(Box::new(Node::new(value)));
                }
            }
        } else if value > node.value {
            match node.right {
                Some(ref mut n) => {
                    BinaryTree::insert_helper(n, value);
                },
                None => {
                    node.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    pub fn insert(&mut self, value: i32) {
        match self.root {
            Some(ref mut n) => {
                BinaryTree::insert_helper(n, value);
            },
            None => {
                self.root = Some(Box::new(Node::new(value))); 
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            _ => false,
        }
    }

    fn search_helper(node: &Node, search_value: i32) -> Option<&Node> {
        if search_value < node.value {
            match node.left {
                Some(ref n) => BinaryTree::search_helper(n, search_value),
                None => None
            }
        } else if search_value > node.value {
            match node.right {
                Some(ref n) => BinaryTree::search_helper(n, search_value),
                None => None
            }
        } else {
            Some(node)
        }
    }

    pub fn search(&self, search_value: i32) -> Option<&Node> {
        match self.root {
            Some(ref n) => {
                BinaryTree::search_helper(n, search_value)
            },
            None => None
        } 
    }

    pub fn min(&mut self, mut index: i32) -> Option<&Node>  {
        let mut vec = vec![]; //Node 
        let mut current = &self.root;
      
        while vec.len() > 0 || !current.is_none() { 
            match current {
                Some(ref n) => {
                    vec.push(n);
                    current = &current.unwrap().left;
                },
                None => {
                    current = &vec.pop();
                    index -= 1;
                    if index == 0 {
                        return Some(current);
                    }
                    current = &current.unwrap().right;
                }
            }
        }

        /*
        while vec.len() > 0 || !(&current).is_none() {
            if !current.is_none() {
                vec.push(&current);
                current = &(&current.unwrap()).left;
            } else {
                current = &vec.pop().unwrap();
                index -= 1;
                if index == 0 { 
                    return &current;
                }

                current = &(&current.unwrap()).right;
            }
        }
        */
        return None;
    }
    /*
    fn left_rotation(&self, value: i32) -> Option<&Node> {
    
    }

    fn right_rotation(&self, value: i32) -> Option<&Node> {
    
    }
    
    pub fn balance(&self) {
        let mut current = self.root;
        while !current.is_none() {
            if !current.is_none() {
                let node = self.right_rotation(current.unwrap().value);
                current = node;
            } else {
                current = current.unwrap().right;
            }
        }
    }
    */

}

