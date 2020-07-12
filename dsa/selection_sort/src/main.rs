
extern crate rand;
use rand::prelude::*;


pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}


fn main() {

  let mut rng = rand::thread_rng();

  let mut nums:Vec<i32> = (1..100).collect();

  nums.shuffle(&mut rng);

  selection_sort(&mut nums);

  for i in 0..nums.len() - 1 {
    assert!(nums[i] <= nums[i + 1]);
  }

  let mut chars:Vec<i32> = ('a'..'Z').collect();
}
