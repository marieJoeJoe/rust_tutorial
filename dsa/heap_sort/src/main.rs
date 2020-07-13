
extern crate rand;
use rand::prelude::*;

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

/// Convert `arr` into a max heap.
fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

/// Move the element at `root` down until `arr` is a max heap again.
///
/// This assumes that the subtrees under `root` are valid max heaps already.
fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}
fn main() {

  let mut rng = rand::thread_rng();

  let mut nums:Vec<i32> = (1..100).collect();

  nums.shuffle(&mut rng);

  heap_sort(&mut nums);

  for i in 0..nums.len() - 1 {
    assert!(nums[i] <= nums[i + 1]);
  }
}
