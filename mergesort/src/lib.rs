use rand::{Rng, thread_rng};

pub trait Vector<I32> { 
    fn new(number: usize, min: i32, max: i32) ->  Self where Self: Sized;
    fn len(&self) -> isize; 
    fn display(&self);
    fn display_from_to(&self, l: usize, r: usize);
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

    fn display_from_to(&self, l: usize, r: usize) {
        print!("[ ");
        for i in l..=r {
            print!("{}: {} ", i, self[i]);
        }
        println!("]");
    }
 
    fn swap(&mut self, a: isize, b: isize) {
        let tmp = self[a as usize];
        self[a as usize] = self[b as usize];
        self[b as usize] = tmp;
    }

    /*
    fn merge(&mut self, l: isize, sep: isize, r: isize) {
        let mut left = l;
        let mut right = sep + 1;
        let mut scratch_index = l;

        let mut scratch: Vec<i32> = vec![0i32; self.len()];

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
        
        for i in l..=r {
            self[i as usize] = scratch[i as usize];
        }
    }
    */ 
    fn merge(&mut self, l: isize, sep:isize, r:isize) {
        let left_vec = self[l as usize..=sep as usize].to_vec();
        let mut left = left_vec.iter().peekable();   
        let right_vec = self[sep as usize + 1..=r as usize].to_vec();
        let mut right = right_vec.iter().peekable();

        for k in l..=r {
            if left.peek().is_none() {           
                self[k as usize] = *right.next().unwrap(); 
                continue;       
            }       

            if right.peek().is_none() {           
                self[k as usize] = *left.next().unwrap();           
                continue;       
            } 

            if right.peek().unwrap() < left.peek().unwrap() {           
                self[k as usize] = *right.next().unwrap();       
            } else {           
                self[k as usize] = *left.next().unwrap();       
            }   
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
            while j > l && self[j as usize - 1] > value {
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
                let sep = (l + r) / 2;
                self.hybrid_sort(l, sep, k);
                self.hybrid_sort(sep + 1, r, k);
                self.merge(l, sep, r);
            } else {
                self.insertion_sort(l, r);
            }
        }
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
    fn test_merge_8_elements() {
        let mut vec1: Vec<i32> = vec![-1, 2, 4, 6];
        let mut vec2: Vec<i32> = vec![0, 1, 5, 7];
        let assert_vec: Vec<i32> = vec![-1, 0, 1, 2, 4, 5, 6, 7];
        vec1.append(&mut vec2);
        vec1.merge(0, 3, vec1.len() as isize - 1);
        assert_eq!(vec1, assert_vec);
    }

    #[test]
    fn test_merge_9_elements() {
        let mut vec1: Vec<i32> = vec![-1, 2, 4, 6, 7];
        let mut vec2: Vec<i32> = vec![-8, 0, 1, 5];
        let assert_vec: Vec<i32> = vec![-8, -1, 0, 1, 2, 4, 5, 6, 7];
        vec1.append(&mut vec2);
        vec1.merge(0, 4, vec1.len() as isize - 1);
        assert_eq!(vec1, assert_vec);
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
        let mut vec: Vec<i32> = Vector::new(1600, 0, 1000);
        vec.display();
        vec.hybrid_sort(0, vec.len() as isize - 1, 32);
        vec.display();
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        } 
    }

    #[test]
    fn test_mergesort_10_6() {
        let mut vec: Vec<i32> = Vector::new(1_000_000, 0, 1_00_000);
        vec.display();
        vec.mergesort(0, vec.len() as isize - 1);
        vec.display();
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }  
    }

    #[test]
    fn test_mergesort_10_7() {
        let mut vec: Vec<i32> = Vector::new(10_000_000, 0, 1_00_000);
        vec.mergesort(0, vec.len() as isize - 1);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }  
    }

    #[test]
    fn test_hybrid_sort_10_6() {
        let mut vec: Vec<i32> = Vector::new(1_000_000, 0, 1_00_000);
        vec.hybrid_sort(0, vec.len() as isize - 1, 32);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }  
    }

    #[test]
    fn test_hybrid_sort_10_7() {
        let mut vec: Vec<i32> = Vector::new(10_000_000, 0, 1_00_000);
        vec.display();
        vec.hybrid_sort(0, vec.len() as isize - 1, 17);
        vec.display();
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }  
    }
}

