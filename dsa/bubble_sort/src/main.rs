

extern crate rand;
use rand::prelude::*;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn main() {

  let mut rng = rand::thread_rng();

  let mut nums:Vec<i32> = (1..100).collect();

  nums.shuffle(&mut rng);

  bubble_sort(&mut nums);    

  for i in 0..nums.len() - 1 {
    assert!(nums[i] <= nums[i + 1]);
  }

}
