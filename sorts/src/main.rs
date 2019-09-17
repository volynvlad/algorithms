use::sorts::Vector;

fn main() {
    let number: usize = 6;
    let min = 7;
    let max = 40;
    let mut array: Vec<i32> = Vector::new(number, min, max);
    let mut array2 = array.clone();
    println!("right quick sort"); 
    array.display();
    array.right_quick_sort(0usize, array.len() as usize - 1 );
    array.display();
    println!("random quick sort");
    array2.display();
    array2.random_quick_sort(0usize, array2.len() as usize - 1 );
    array2.display();
}
