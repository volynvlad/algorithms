use::sorts::Vector;
use std::time::Instant;

fn get_time(vec: Vec<i32>) {
    let mut vec_right_qs = vec.clone();
    let mut vec_random_qs = vec.clone();
    println!("Copied!");

    let t1 = Instant::now(); 
    vec_right_qs.right_quick_sort(0usize, vec_right_qs.len() - 1);
    let t2 = Instant::now();
    println!("Right qs {:?}", t2.duration_since(t1));
    
    let t3 = Instant::now(); 
    vec_random_qs.random_quick_sort(0usize, vec_random_qs.len() - 1);
    let t4 = Instant::now();
    println!("Random qs {:?}", t4.duration_since(t3));
}

fn main() {
    
    let t1 = Instant::now();
    let vec_5_5: Vec<i32> = Vector::new(1_000_00, 0, 1_000_00);
    let vec_5_6: Vec<i32> = Vector::new(1_000_00, 0, 1_000_000);
    let vec_6_5: Vec<i32> = Vector::new(1_000_000, 0, 1_000_00);
    let vec_6_6: Vec<i32> = Vector::new(1_000_000, 0, 1_000_000);
    let vec_7_5: Vec<i32> = Vector::new(1_000_0000, 0, 1_000_00);
    let vec_7_6: Vec<i32> = Vector::new(1_000_0000, 0, 1_000_000);
    let t2 = Instant::now();
    println!("Randomed - {:?}", t2.duration_since(t1));
    
    println!("{} elements with range {}", 1_000_00, 1_000_00);
    get_time(vec_5_5);
    println!("{} elements with range {}", 1_000_00, 1_000_000);
    get_time(vec_5_6);
    println!("{} elements with range {}", 1_000_000, 1_000_00);
    get_time(vec_6_5);
    println!("{} elements with range {}", 1_000_000, 1_000_000);
    get_time(vec_6_6);
    println!("{} elements with range {}", 1_000_0000, 1_000_00);
    get_time(vec_7_5);
    println!("{} elements with range {}", 1_000_0000, 1_000_000);
    get_time(vec_7_6);
}
