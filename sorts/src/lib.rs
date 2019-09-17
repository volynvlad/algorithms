use std::time;
use rand::{Rng, thread_rng};

pub trait Vector<I32> {
    fn new(number: usize, min: i32, max: i32) ->  Self where Self: Sized;
    fn len(&self) -> usize; 
    fn display(&self);
    fn swap(&mut self, a: usize, b: usize);

    fn right_partition(&mut self, l: usize, r: usize) -> usize;
    fn random_partition(&mut self, l: usize, r: usize) -> usize;

    fn right_quick_sort(&mut self, l: usize, r: usize);
    fn random_quick_sort(&mut self, l: usize, r:usize);
    fn insert_sort(&mut self);
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
        let mut i = l; 

        for j in l..r {
            if self[j] < pivot {
                self.swap(i, j);
                i = i + 1;
            }
        }

        self.swap(i, r);
        i
    }

    fn random_partition(&mut self, l: usize, r: usize) -> usize {
        let mut rng = thread_rng();
        let rand = rng.gen_range(l, r);
        let pivot = self[rand];
        let mut i = l; 
        let mut k = r;

        for j in l..r {
            if self[j] < pivot {
                self.swap(i, j);
                i = i + 1;
            } else {
                //self.swap(k, j);
                //k = k - 1;
            }
        }

        self.swap(i, r);
        i
    }

    fn right_quick_sort(&mut self, l: usize, r: usize) {
        if l < r {
            let sep = self.right_partition(l, r);
            if sep == 0 {
                return;
            }
            self.right_quick_sort(l, sep - 1);
            self.right_quick_sort(sep + 1, r);
        }
    }

    fn random_quick_sort(&mut self, l: usize, r: usize) { 
        if l < r {
            let sep = self.random_partition(l, r);
            if sep == 0 {
                return;
            }
            self.random_quick_sort(l, sep - 1);
            self.random_quick_sort(sep + 1, r);
        }
    }

    fn insert_sort(&mut self) {

    }
}

