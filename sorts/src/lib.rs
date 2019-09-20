use rand::{Rng, thread_rng};

pub trait Vector<I32> {
    fn new(number: usize, min: i32, max: i32) ->  Self where Self: Sized;
    fn len(&self) -> usize; 
    fn display(&self);
    fn swap(&mut self, a: usize, b: usize);

    fn right_partition(&mut self, l: usize, r: usize) -> usize;
    fn random_partition(&mut self, l: usize, r: usize) -> usize;

    fn right_quick_sort(&mut self, l: usize, r: usize);
    fn random_quick_sort(&mut self, l: usize, r: usize);
    fn insertion_sort(&mut self, l: usize, r: usize);
    fn hybrid_sort(&mut self, l: usize, r: usize, k: usize);
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

    fn len(&self) -> usize {
        self.len()
    }

    fn display(&self) {
        println!("{:?}", self);
    }
 
    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self[a];
        self[a] = self[b];
        self[b] = tmp;
    }

    fn right_partition(&mut self, l: usize, r: usize) -> usize {
        let pivot = self[r];
        let i: isize = l as isize; 
        let mut i: isize = i - 1;

        for j in l..r {
            if self[j] < pivot { 
                i = i + 1;
                self.swap(i as usize, j);
            }
        }

        self.swap((i + 1) as usize, r);
        (i + 1) as usize
    }

    fn random_partition(&mut self, l: usize, r: usize) -> usize {
        let mut rng = thread_rng();
        
        let pivot = self[rng.gen_range(l, r)];
        let mut i = l;
        let mut j = r;
        
        loop {
            while self[i] < pivot {
                i = i + 1;
            }

            while self[j] > pivot {
                j = j - 1;
            }
            
            if i >= j { break; }
            
            self.swap(i, j);
            
            if self[i] == self[j] {
                i = i + 1;    
            }
        }
        j
    }

    fn right_quick_sort(&mut self, l: usize, r: usize) {
        if l < r { 
            let sep = self.right_partition(l, r);
            if sep != 0 {
                self.right_quick_sort(l, sep - 1);
            }
            self.right_quick_sort(sep + 1, r);
        }
    }

    fn random_quick_sort(&mut self, l: usize, r: usize) { 
        if l < r {
            let sep = self.random_partition(l, r);
            if sep != 0 { 
                self.random_quick_sort(l, sep - 1);
            }
            self.random_quick_sort(sep + 1, r);
        }
    }

    fn insertion_sort(&mut self, l: usize, r: usize) {
        for i in l..r {
            for j in (l..i).rev() {
                if self[j] >= self[j + 1] {
                    self.swap(j, j + 1);
                } else {
                    break;
                }
            }
        }
    }

    fn hybrid_sort(&mut self, l: usize, r: usize, k: usize) {
        if l < r {
            if (r - l + 1) > k {    
                let sep = self.right_partition(l, r);
                if sep != 0 { 
                    self.hybrid_sort(l, sep - 1, k);
                }
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
        let mut vec: Vec<i32> = Vector::new(9, 1, 4);
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
        let mut vec: Vec<i32> = Vector::new(1_000usize, 0, 1000);
        vec.right_quick_sort(0usize, vec.len() as usize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_random_qs() {
        let mut vec: Vec<i32> = Vector::new(1_000usize, 0, 1000);
        vec.random_quick_sort(0usize, vec.len() as usize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_insertion_sort() {        
        let mut vec: Vec<i32> = Vector::new(1_000usize, 0, 1000);
        vec.insertion_sort(0, 1000);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn test_hybrid_sort() { 
        let mut vec: Vec<i32> = Vector::new(1_000usize, 0, 1000);
        vec.hybrid_sort(0usize, vec.len() as usize - 1, 10);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}

