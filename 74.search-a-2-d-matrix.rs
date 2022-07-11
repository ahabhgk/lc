/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 *
 * https://leetcode.com/problems/search-a-2d-matrix/description/
 *
 * algorithms
 * Medium (45.36%)
 * Likes:    8409
 * Dislikes: 285
 * Total Accepted:    869K
 * Total Submissions: 1.9M
 * Testcase Example:  '[[1,3,5,7],[10,11,16,20],[23,30,34,60]]\n3'
 *
 * Write an efficient algorithm that searches for a value target in an m x n
 * integer matrix matrix. This matrix has the following properties:
 *
 *
 * Integers in each row are sorted from left to right.
 * The first integer of each row is greater than the last integer of the
 * previous row.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 100
 * -10^4 <= matrix[i][j], target <= 10^4
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for row in matrix {
      if row[0] <= target && target <= row[row.len() - 1] {
        for cell in row {
          if cell == target {
            return true;
          }
        }
      }
    }
    false
  }
}
// @lc code=end
