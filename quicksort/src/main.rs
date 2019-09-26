use std::time::Instant;

use::quicksort::Vector;

fn get_time(vec: Vec<i32>) { 
    extern crate quickersort;
    
    let mut vec_right_qs = vec.clone();
    let mut vec_random_qs = vec.clone();
    let mut vec_lib = vec.clone();

    let t1 = Instant::now();
    vec_right_qs.right_quick_sort(0, vec_right_qs.len() as isize - 1);
    let t2 = Instant::now();
    println!("Right  qs {:?}", t2.duration_since(t1));
 
    let t3 = Instant::now();
    vec_random_qs.random_quick_sort(0, vec_random_qs.len() as isize - 1);
    let t4 = Instant::now();
    println!("Random qs {:?}", t4.duration_since(t3));

    for k in 4..13 {
        let mut vec_hybrid_s = vec.clone();
        let t5 = Instant::now(); 
        vec_hybrid_s.hybrid_sort(0, vec_hybrid_s.len() as isize - 1, k);
        let t6 = Instant::now();
        println!("Hybrid  s {:?} with k = {}", t6.duration_since(t5), k);
    }

    let t5 = Instant::now();
    vec_lib.sort();
    let t6 = Instant::now();
    println!("lib sort {:?}", t6.duration_since(t5));
}

fn main() {
   
    let mut i: u32 = 5;
    let mut j;
    let base: i32 = 10;
    while i <= 7 {
        j = 5u32;
        while j <= 7 {
            let mut vec: Vec<i32> = Vector::new(base.pow(i) as usize, 0, base.pow(j));
            println!("10^{} elements with range 10^{}", i, j);
            get_time(vec);
            j = j + 1;
        }
        i = i + 1;
    }
}
