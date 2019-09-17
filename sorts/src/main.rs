use::sorts::Vector;
use std::time::{Instant, SystemTime};

fn main() {
    //let number: usize = 6;
    //let min = 7;
    //let max = 40;
    //let mut array: Vec<i32> = Vector::new(number, min, max);
    //let mut array2 = array.clone();
    //println!("right quick sort"); 
    //array.display();
    //array.right_quick_sort(0usize, array.len() as usize - 1);
    //array.display();
    //println!("random quick sort");
    //array2.display();
    //array2.random_quick_sort(0usize, array2.len() as usize - 1 ); 
    //array2.display();
    let mut array3: Vec<i32> = Vector::new(1_000_000, 0, 1_000);
    let mut array4: Vec<i32> = Vector::new(1_000_000, 0, 1_000);
    
    let t1 = Instant::now(); 
    array3.right_quick_sort(0usize, 999_999usize);
    let t2 = Instant::now();

    let t3 = Instant::now(); 
    array4.right_quick_sort(0usize, 999_999usize);
    let t4 = Instant::now();

    println!("{:?}", t2.duration_since(t1));
    println!("{:?}", t4.duration_since(t3));
}
