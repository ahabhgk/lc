/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 *
 * https://leetcode.com/problems/contains-duplicate/description/
 *
 * algorithms
 * Easy (61.03%)
 * Likes:    5409
 * Dislikes: 973
 * Total Accepted:    1.8M
 * Total Submissions: 2.9M
 * Testcase Example:  '[1,2,3,1]'
 *
 * Given an integer array nums, return true if any value appears at least twice
 * in the array, and return false if every element is distinct.
 *
 *
 * Example 1:
 * Input: nums = [1,2,3,1]
 * Output: true
 * Example 2:
 * Input: nums = [1,2,3,4]
 * Output: false
 * Example 3:
 * Input: nums = [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

pub struct Solution;

// @lc code=start
use std::collections::HashSet;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
      if set.get(&num).is_some() {
        return true;
      }
      set.insert(num);
    }
    false
  }
}
// @lc code=end
