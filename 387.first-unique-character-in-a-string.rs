/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 *
 * https://leetcode.com/problems/first-unique-character-in-a-string/description/
 *
 * algorithms
 * Easy (57.48%)
 * Likes:    5302
 * Dislikes: 205
 * Total Accepted:    1.1M
 * Total Submissions: 1.9M
 * Testcase Example:  '"leetcode"'
 *
 * Given a string s, find the first non-repeating character in it and return
 * its index. If it does not exist, return -1.
 *
 *
 * Example 1:
 * Input: s = "leetcode"
 * Output: 0
 * Example 2:
 * Input: s = "loveleetcode"
 * Output: 2
 * Example 3:
 * Input: s = "aabb"
 * Output: -1
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^5
 * s consists of only lowercase English letters.
 *
 *
 */

pub struct Solution;

// @lc code=start
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
  pub fn first_uniq_char(s: String) -> i32 {
    let mut map = HashMap::new();
    for c in s.chars() {
      match map.entry(c) {
        Entry::Occupied(mut e) => *e.get_mut() += 1,
        Entry::Vacant(e) => {
          e.insert(1);
        }
      };
    }
    for (i, c) in s.char_indices() {
      if Some(&1) == map.get(&c) {
        return i as i32;
      }
    }
    -1
  }
}
// @lc code=end

#[test]
fn smock() {
  assert_eq!(Solution::first_uniq_char("leetcode".to_owned()), 0);
  assert_eq!(Solution::first_uniq_char("loveleetcode".to_owned()), 2);
}
