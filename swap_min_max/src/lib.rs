pub trait Array {
    fn swap_min_max(&mut self);
    fn display(&self);
}

impl Array for [i32] {
    fn swap_min_max<>(&mut self) {
        let mut min_index = 0; // 1
        let mut max_index = 0; // 1
        let length = self.len(); // 1
        for i in 1..length { // 1 + 2(n - 1)
            if self[min_index] > self[i] { // 3(n - 1)
                min_index = i; // 3(n - 1)
            } else if self[max_index] < self[i] { // 3(n - 1)
                max_index = i; // 3(n - 1)
            }
        }
        let tmp: i32 = self[min_index]; // 2
        self[min_index] = self[max_index]; // 3
        self[max_index] = tmp; // 2
    }

    fn display(&self) {
        for i in 0..self.len() { 
            print!("{} ", self[i]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut array: [i32; 5] = [4, 9, 1, 7, 2];
        array.swap_min_max();
        assert_eq!(
            [4, 1, 9, 7, 2],
            array
        );
    }

    #[test]
    fn test2() {
        let mut array: [i32; 8] = [1; 8];
        array.swap_min_max();
        assert_eq!(
            [1i32; 8],
            array
        );
    }
}

