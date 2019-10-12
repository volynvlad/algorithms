use super::*;

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

