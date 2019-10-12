mod vector;
mod binary_search_tree;
use vector::Vector;
use binary_search_tree::BSTNode;

fn main() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let search_value = 2;
    let result_value = vec.interpolation_search(search_value, 0, vec.len() as i32 - 1).unwrap();
    println!("{}", result_value);

    let tree = BSTNode::new();
}

#[cfg(test)]
mod tests;
