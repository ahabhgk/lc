/*
 * @lc app=leetcode id=350 lang=rust
 *
 * [350] Intersection of Two Arrays II
 *
 * https://leetcode.com/problems/intersection-of-two-arrays-ii/description/
 *
 * algorithms
 * Easy (55.20%)
 * Likes:    4782
 * Dislikes: 728
 * Total Accepted:    844.5K
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,2,2,1]\n[2,2]'
 *
 * Given two integer arrays nums1 and nums2, return an array of their
 * intersection. Each element in the result must appear as many times as it
 * shows in both arrays and you may return the result in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2,2]
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [4,9]
 * Explanation: [9,4] is also accepted.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums1.length, nums2.length <= 1000
 * 0 <= nums1[i], nums2[i] <= 1000
 *
 *
 *
 * Follow up:
 *
 *
 * What if the given array is already sorted? How would you optimize your
 * algorithm?
 * What if nums1's size is small compared to nums2's size? Which algorithm is
 * better?
 * What if elements of nums2 are stored on disk, and the memory is limited such
 * that you cannot load all elements into the memory at once?
 *
 *
 */

pub struct Solution;

// @lc code=start
use std::cmp::Ordering;

impl Solution {
  pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort_unstable();
    nums2.sort_unstable();
    let mut i1 = 0;
    let mut i2 = 0;
    let mut res = Vec::new();
    while i1 < nums1.len() && i2 < nums2.len() {
      match nums1[i1].cmp(&nums2[i2]) {
        Ordering::Less => i1 += 1,
        Ordering::Greater => i2 += 1,
        Ordering::Equal => {
          res.push(nums1[i1]);
          i1 += 1;
          i2 += 1;
        }
      }
    }
    res
  }
}
// @lc code=end
