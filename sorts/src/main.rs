use std::time::Instant;

use::sorts::Vector;

fn get_time(vec: Vec<i32>) { 
    let mut vec_right_qs = vec.clone();
    let mut vec_random_qs = vec.clone();
    println!("Copied!");

    let t1 = Instant::now(); 
    vec_right_qs.right_quick_sort(0usize, vec_right_qs.len() - 1);
    let t2 = Instant::now();
    println!("Right  qs {:?}", t2.duration_since(t1));
 
    let t3 = Instant::now(); 
    vec_random_qs.random_quick_sort(0usize, vec_random_qs.len() - 1);
    let t4 = Instant::now();
    println!("Random qs {:?}", t4.duration_since(t3));

    for k in 6u32..13u32 { 
        let mut vec_hybrid_s = vec.clone();
        let t5 = Instant::now(); 
        vec_hybrid_s.hybrid_sort(0usize, vec_hybrid_s.len() - 1, k);
        let t6 = Instant::now();
        println!("Hybrid  s {:?} with k = {}", t6.duration_since(t5), k);
    }
}

fn main() {
   
    let vec_5_5: Vec<i32> = Vector::new(1_000_00, 0, 1_000_00);
    let vec_5_6: Vec<i32> = Vector::new(1_000_00, 0, 1_000_000);
    let vec_5_7: Vec<i32> = Vector::new(1_000_00, 0, 1_000_0000);
    let vec_6_5: Vec<i32> = Vector::new(1_000_000, 0, 1_000_00);
    let vec_6_6: Vec<i32> = Vector::new(1_000_000, 0, 1_000_000);
    let vec_6_7: Vec<i32> = Vector::new(1_000_000, 0, 1_000_0000);
    let vec_7_5: Vec<i32> = Vector::new(1_000_0000, 0, 1_000_00);
    let vec_7_6: Vec<i32> = Vector::new(1_000_0000, 0, 1_000_000);
    let vec_7_7: Vec<i32> = Vector::new(1_000_0000, 0, 1_000_0000);
    
    println!("10^5 elements with range 10^5");
    get_time(vec_5_5);
    println!("10^5 elements with range 10^6");
    get_time(vec_5_6);
    println!("10^5 elements with range 10^7");
    get_time(vec_5_7);
    println!("10^6 elements with range 10^5");
    get_time(vec_6_5);
    println!("10^6 elements with range 10^6");
    get_time(vec_6_6);
    println!("10^6 elements with range 10^7");
    get_time(vec_6_7);
    println!("10^7 elements with range 10^5");
    get_time(vec_7_5);
    println!("10^7 elements with range 10^6");
    get_time(vec_7_6); 
    println!("10^7 elements with range 10^7");
    get_time(vec_7_7);
}
