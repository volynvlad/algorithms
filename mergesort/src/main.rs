use mergesort::Vector;
extern crate mergesort;

fn main() {
    let mut vec: Vec<i32> = Vector::new(16, 0, 1000);
    vec.display();
    vec.hybrid_sort(0, vec.len() as isize - 1, 3);
}
