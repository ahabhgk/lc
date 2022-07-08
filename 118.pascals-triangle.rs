/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 *
 * https://leetcode.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (65.94%)
 * Likes:    6393
 * Dislikes: 224
 * Total Accepted:    857.1K
 * Total Submissions: 1.3M
 * Testcase Example:  '5'
 *
 * Given an integer numRows, return the first numRows of Pascal's triangle.
 *
 * In Pascal's triangle, each number is the sum of the two numbers directly
 * above it as shown:
 *
 *
 * Example 1:
 * Input: numRows = 5
 * Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
 * Example 2:
 * Input: numRows = 1
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 *
 * 1 <= numRows <= 30
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
    res.push(vec![1]);
    for i in 1..num_rows {
      let mut row = Vec::with_capacity(i + 1);
      let pre = &res[i - 1];
      for j in 0..=i {
        row.push(
          if j == 0 { 0 } else { pre[j - 1] } + if j == i { 0 } else { pre[j] },
        );
      }
      res.push(row);
    }
    res
  }
}
// @lc code=end
