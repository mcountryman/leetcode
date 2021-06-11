use std::collections::{hash_map::Entry, HashMap};

pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = HashMap::<i32, Vec<usize>>::with_capacity(nums.len());

    for (idx, num) in nums.iter().cloned().enumerate() {
      match indices.entry(num) {
        Entry::Vacant(entry) => {
          entry.insert(vec![idx]);
        }
        Entry::Occupied(mut entry) => {
          entry.get_mut().push(idx);
        }
      };
    }

    for (i, a) in nums.iter().cloned().enumerate() {
      let b = target - a;
      let idx = indices.get(&b);
      match idx {
        Some(idx) => {
          for j in idx.iter().cloned() {
            if j != i {
              return vec![i as _, j as _];
            }
          }
        }
        None => continue,
      }
    }

    return vec![-1, -1];
  }
}

fn main() {
  assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
  assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
