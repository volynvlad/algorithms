use super::*;
use super::BinaryTree;

#[test]
fn test_len() {
    let vec: Vec<i32> = vec![8; 8];
    assert!(vec.len() == 8);
}

#[test]
fn test_sequential_search() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let assert_value = Some(2);
    let search_value = 5;
    let result_value = vec.sequential_search(search_value);
    assert_eq!(result_value, assert_value, "result is {}", result_value.unwrap());
}

#[test]
fn test_binary_search() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let assert_value = Some(2);
    let search_value = 5;
    let result_value = vec.binary_search(search_value, 0, vec.len() as i32 - 1);
    assert_eq!(result_value, assert_value, "result is {}", result_value.unwrap());
}

#[test]
fn test_interpolation_search() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let assert_value = Some(2);
    let search_value = 5;
    let result_value = vec.interpolation_search(search_value, 0, vec.len() as i32 - 1);
    assert_eq!(result_value, assert_value, "result is {}", result_value.unwrap());
}

#[test]
fn test_sequential_search_none() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let search_value = 3;
    let result_value = vec.sequential_search(search_value);
    assert!(result_value.is_none(), "result is {}", result_value.unwrap());
}

#[test]
fn test_binary_search_none() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let search_value = 3;
    let result_value = vec.binary_search(search_value, 0, vec.len() as i32 - 1);
    assert!(result_value.is_none(), "result is {}", result_value.unwrap());
}

#[test]
fn test_interpolation_search_none() {
    let vec = vec![1, 2, 5, 6, 7, 9, 20];
    let search_value = 3;
    let result_value = vec.interpolation_search(search_value, 0, vec.len() as i32 - 1);
    assert!(result_value.is_none(), "result is {}", result_value.unwrap());
}

#[test]
fn test_is_empty() {
    let tree = BinaryTree::new();
    assert!(tree.is_empty());
}

#[test]
fn test_is_not_empty() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    assert!(!tree.is_empty());
}

#[test]
fn test_find() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);
    let search_value = 4;
    println!("{:?}", tree.search(search_value));
    assert!(tree.search(search_value).unwrap().value == search_value);
}

#[test]
fn test_find_none() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);
    let search_value = 2;
    assert!(tree.search(search_value).is_none());
}

#[test] 
fn test_min() { 
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    tree.insert(4);
    tree.insert(3);
    let index = 4;
    assert_eq!(tree.min(4), 6);
}


