use swap_min_max::Array;

fn main() {
    let mut array: [i32; 6] = [4, 5, 9, 1, 3, 8];
    array.display();
    array.swap_min_max();
    array.display();
}
