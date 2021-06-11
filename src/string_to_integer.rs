use std::{cmp, collections::HashMap};

pub struct Solution;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut beg = 0;
    let mut end = 0;
    let mut max = 0;

    let mut seen = HashMap::with_capacity(s.len());
    let mut seen_rev = HashMap::with_capacity(s.len());

    for (i, ch) in s.chars().enumerate() {
      match seen.get(&ch).cloned() {
        Some(idx) => {
          for i in beg..idx {
            let chr = seen_rev.remove(&i).unwrap();
            seen.remove(&chr).unwrap();
          }

          beg = idx + 1;
          end += 1;
        }
        None => end += 1,
      };

      max = cmp::max(end - beg, max);

      seen.insert(ch, i);
      seen_rev.insert(i, ch);
    }

    max as _
  }
}

fn main() {
  assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);

  assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);

  assert_eq!(
    Solution::length_of_longest_substring("advdfa".to_string()),
    4
  );

  assert_eq!(
    Solution::length_of_longest_substring("abcabcbb".to_string()),
    3
  );

  assert_eq!(
    Solution::length_of_longest_substring("bbbbb".to_string()),
    1
  );

  assert_eq!(
    Solution::length_of_longest_substring("pwwkew".to_string()),
    3
  );

  assert_eq!(
    Solution::length_of_longest_substring("abcdefghijklmnopqrstuvwxyz".to_string()),
    26
  );
}
