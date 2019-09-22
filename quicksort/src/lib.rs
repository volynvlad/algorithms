use rand::{Rng, thread_rng};

pub trait Vector<I32> {
    fn new(number: usize, min: i32, max: i32) ->  Self where Self: Sized;
    fn len(&self) -> isize; 
    fn display(&self);
    fn swap(&mut self, a: isize, b: isize);

    fn right_partition(&mut self, l: isize, r: isize) -> isize;
    fn random_partition(&mut self, l: isize, r: isize) -> isize;

    fn right_quick_sort(&mut self, l: isize, r: isize);
    fn random_quick_sort(&mut self, l: isize, r: isize);
    fn insertion_sort(&mut self, l: isize, r: isize);
    fn hybrid_sort(&mut self, l: isize, r: isize, k: isize);
}

impl Vector<i32> for Vec<i32> {

    fn new(number: usize, min: i32, max: i32) -> Self {
        let mut rng = thread_rng();
        let mut ret: Vec<i32> = Vec::with_capacity(number);
        for _ in 0..ret.capacity() {
            ret.push(rng.gen_range(min, max));
        }
        ret
    }

    fn len(&self) -> isize {
        self.len() as isize
    }

    fn display(&self) {
        println!("{:?}", self);
    }
 
    fn swap(&mut self, a: isize, b: isize) {
        let tmp = self[a as usize];
        self[a as usize] = self[b as usize];
        self[b as usize] = tmp;
    }

    fn right_partition(&mut self, l: isize, r: isize) -> isize {
        let pivot = self[r as usize];
        let mut i = l - 1;

        for j in l..r {
            if self[j as usize] < pivot { 
                i = i + 1;
                self.swap(i, j);
            }
        }

        self.swap((i + 1), r);
        (i + 1) 
    }

    fn random_partition(&mut self, l: isize, r: isize) -> isize {
        let mut rng = thread_rng();
        
        let pivot = self[rng.gen_range(l, r) as usize];
        let mut i = l;
        let mut j = r;
        
        loop {
            while self[i as usize] < pivot {
                i = i + 1;
            }

            while self[j as usize] > pivot {
                j = j - 1;
            }
            
            if i >= j { break; }
            
            self.swap(i, j);
            
            if self[i as usize] == self[j as usize] {
                i = i + 1;    
            }
        }
        j
    }

    fn right_quick_sort(&mut self, l: isize, r: isize) {
        if l < r { 
            let sep = self.right_partition(l, r);
            self.right_quick_sort(l, sep - 1);
            self.right_quick_sort(sep + 1, r);
        }
    }

    fn random_quick_sort(&mut self, l: isize, r: isize) { 
        if l < r {
            let sep = self.random_partition(l, r);
            self.random_quick_sort(l, sep - 1);
            self.random_quick_sort(sep + 1, r);
        }
    }

    fn insertion_sort(&mut self, l: isize, r: isize) {
        let mut value;
        let mut i = l;
        let mut j;
        while i <= r { // 1 + 2n
            /*
            for j in (l..i).rev() { // 1 + 2m
                if self[j as usize] >= self[(j + 1) as usize] { // 3m*n
                    self.swap(j, j + 1); // 4m*n
                } else {
                    break;
                }
            }
            */ 
            value = self[i as usize];
            j = i;
            while j > 0 && j >= l && self[j as usize - 1] > value {
                self[j as usize] = self[j as usize - 1];
                j -= 1;
            }
            self[j as usize] = value;
            i += 1;
        }
    }

    fn hybrid_sort(&mut self, l: isize, r: isize, k: isize) {
        if l < r {
            if (r - l) > k {    
                let sep = self.right_partition(l, r);
                self.hybrid_sort(l, sep - 1, k);
                self.hybrid_sort(sep + 1, r, k);
            } else {
                self.insertion_sort(l, r);
            }
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    
    #[test] 
    fn test_len() {
        let vec: Vec<i32> = Vector::new(9, 1, 4);
        assert!(vec.len() == 9);
    }

    #[test]
    fn test_swap() {
        let mut vec: Vec<i32> = vec![1, 3, 0, 8, 2];
        vec.swap(0, 3);
        assert!(vec == vec![8, 3, 0, 1, 2]);
    }

    #[test]
    fn test_right_qs() {
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.right_quick_sort(0, vec.len() as isize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_random_qs() {
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.random_quick_sort(0, vec.len() as isize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_insertion_sort() {        
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.insertion_sort(0, 999);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_hybrid_sort() { 
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.hybrid_sort(0, vec.len() as isize- 1, 10);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}

