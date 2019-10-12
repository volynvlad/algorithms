extern crate rand;
pub use rand::Rng;

pub trait Vector<I32> {
    fn new_sorted(number: usize, min_dist: i32, max_dist:i32) -> Self where Self: Sized;
    fn new(number: usize, min_value: i32, max_value: i32) -> Self where Self: Sized;
    fn len(&self) -> isize;
    fn display(&self);

    fn sequential_search(&self, search_num: i32) -> Option<i32>;
    fn binary_search(&self, search_num: i32, l: i32, r: i32) -> Option<i32>;
    fn interpolation_search(&self, search: i32, l: i32, r: i32) -> Option<i32>;
}

impl Vector<i32> for Vec<i32> {
    fn new_sorted(number: usize, min_dist: i32, max_dist:i32) -> Self where Self: Sized {
        let mut rng = rand::thread_rng();
        let mut ret: Vec<i32> = Vec::with_capacity(number);
        ret.push(rng.gen_range(min_dist, max_dist));
        for i in 1..ret.capacity() {
            ret.push(ret[i - 1] + rng.gen_range(min_dist, max_dist));
        }
        ret
    }

    fn new(number: usize, min_value: i32, max_value: i32) -> Self where Self: Sized {
        let mut rng = rand::thread_rng();
        let mut ret: Vec<i32> = Vec::with_capacity(number);
        for _ in 0..ret.capacity() {
            ret.push(rng.gen_range(min_value, max_value));
        }
        ret
    }

    fn len(&self) -> isize {
        self.len() as isize
    }

    fn display(&self) {
        println!("{:?}", self);
    }

    fn sequential_search(&self, search_num: i32) -> Option<i32> {
        for i in 0..self.len() {
            if self[i as usize] == search_num {
                return Some(i as i32);
            }
        }
        return None;
    }

    fn binary_search(&self, search_num: i32, mut l: i32, mut r: i32) -> Option<i32> {
        while l <= r {
            println!("l = {}, r = {}", l, r);
            let mid:i32 = (l + r) / 2;
            if self[mid as usize] == search_num {
                return Some(mid);
            } else if self[mid as usize] > search_num {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return None;
    }

    fn interpolation_search(&self, search_num: i32, mut l: i32, mut r: i32) -> Option<i32> {
        while self[r as usize] != self[l as usize] && l <= r {
            let mid:i32 = l + (search_num - self[l as usize]) / (self[r as usize] - self[l as usize]) * (r - l);
            if self[mid as usize] == search_num {
                return Some(mid);
            } else if self[mid as usize] > search_num {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        if search_num == self[l as usize] {
            return Some(l as i32);        
        } else {
            return None;
        } 
    }
}
