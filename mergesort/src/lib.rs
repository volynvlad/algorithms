use rand::{Rng, thread_rng};

pub trait Vector<I32> { 
    fn new(number: usize, min: i32, max: i32) ->  Self where Self: Sized;
    fn len(&self) -> isize; 
    fn display(&self);
    fn swap(&mut self, a: isize, b: isize);

    fn merge(&mut self, l: isize, sep: isize, r: isize);
    fn mergesort(&mut self, l: isize, r: isize);
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

    fn merge(&mut self, l: isize, sep: isize, r: isize) {
        let mut left = l;
        let mut right = sep + 1;
        let mut scratch_index = l;

        let mut scratch: Vec<i32> = vec![0; self.len()];
        
        while left <= sep && right <= r {
            if self[left as usize] <= self[right as usize] {
                scratch[scratch_index as usize] = self[left as usize];
                left += 1;
            } else {
                scratch[scratch_index as usize] = self[right as usize];
                right += 1;
            }
            scratch_index += 1;
        }

        while left <= sep {
            scratch[scratch_index as usize] = self[left as usize];
            scratch_index += 1;
            left += 1;
        }

        while right <= r {
            scratch[scratch_index as usize] = self[right as usize];
            scratch_index += 1;
            right += 1;
        }
        
        let mut i = l;
        while i <= r {
            self[i as usize] = scratch[i as usize];
            i += 1;
        }
    }

    fn mergesort(&mut self, l: isize, r: isize) {
        if l < r {
            let sep = (l + r) / 2;
            self.mergesort(l, sep);
            self.mergesort(sep + 1, r);
            self.merge(l, sep, r);         
        } 
    }
    
    fn insertion_sort(&mut self, l: isize, r: isize) { 
        let mut value;
        let mut i = l;
        let mut j;
        while i <= r {    
            value = self[i as usize];
            j = i;
            while j > 0 && j >= l && self[j as usize - 1] > value {
                self[j as usize] = self[j as usize - 1];
                j -= 1;
            }
            self[j as usize] = value;
            i += 1;
        } 
        println!("insertion_sort");
        for i in l..=r {
            print!("{} ", self[i as usize]);
        }
        println!("");
    }

    fn hybrid_sort(&mut self, l: isize, r: isize, k: isize) {
        if l < r {
            if (r - l) > k { 
                let sep = (l + r) / 2;
                self.hybrid_sort(l, sep, k);
                self.hybrid_sort(sep + 1, r, k);
                println!("l = {} sep = {} sep + 1 = {} r = {}", self[l as usize], self[sep as usize], self[sep as usize+ 1], self[r as usize]);
                self.merge(l, sep, r);
            } else {
                self.insertion_sort(l, r);
            }
        }
        println!("hybrid_sort");
        for i in l..=r {
            print!("{} ", self[i as usize]);
        }
        println!("");
    }
}

#[cfg(test)]
pub mod tests { 
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
    fn test_mergesort() {
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.mergesort(0, vec.len() as isize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_insertion_sort() {        
        let mut vec: Vec<i32> = Vector::new(1_000, 0, 1000);
        vec.insertion_sort(0, vec.len() as isize - 1);
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

