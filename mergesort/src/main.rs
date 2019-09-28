use mergesort::Vector;
use std::time::Instant;
extern crate mergesort;

fn get_time(vec: Vec<i32>) { 
    
    let mut vec_right_qs = vec.clone();

    let t1 = Instant::now();
    vec_right_qs.mergesort(0, vec_right_qs.len() as isize - 1);
    let t2 = Instant::now();
    println!("merge sort {:?}", t2.duration_since(t1));

    let mut k = 4;
    let mut min_k = k;
    let mut min_time = t2.duration_since(t1);
    while k <= 64{
        let mut vec_hybrid_s = vec.clone();
        let t5 = Instant::now(); 
        vec_hybrid_s.hybrid_sort(0, vec_hybrid_s.len() as isize - 1, k);
        let t6 = Instant::now();
        //println!("Hybrid  s {:?} with k = {}", t6.duration_since(t5), k);
        if min_time > t6.duration_since(t5) {
            min_k = k;
            min_time = t6.duration_since(t5);
        }
        k += 1;
    }
    println!("Best hybrid_sort = {:?} with k = {}", min_time, min_k);
}

fn main() {
   
    let mut i: u32 = 5;
    let mut j;
    let base: i32 = 10;
    while i <= 7 {
        j = 5u32;
        while j <= 7 {
            let vec: Vec<i32> = Vector::new(base.pow(i) as usize, 0, base.pow(j));
            println!("10^{} elements with range 10^{}", i, j);
            get_time(vec);
            j = j + 1;
        }
        i = i + 1;
    }
}
