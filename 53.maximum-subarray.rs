/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 *
 * https://leetcode.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Medium (49.66%)
 * Likes:    22329
 * Dislikes: 1104
 * Total Accepted:    2.5M
 * Total Submissions: 5M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * Given an integer array nums, find the contiguous subarray (containing at
 * least one number) which has the largest sum and return its sum.
 *
 * A subarray is a contiguous part of an array.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution using the divide and conquer approach, which is more subtle.
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pre = 0;
    let mut max = nums[0];
    for num in nums {
      pre = num.max(num + pre);
      max = max.max(pre);
    }
    max
  }
}
// @lc code=end
