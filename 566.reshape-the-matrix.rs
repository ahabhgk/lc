/*
 * @lc app=leetcode id=566 lang=rust
 *
 * [566] Reshape the Matrix
 *
 * https://leetcode.com/problems/reshape-the-matrix/description/
 *
 * algorithms
 * Easy (62.57%)
 * Likes:    2441
 * Dislikes: 274
 * Total Accepted:    259.2K
 * Total Submissions: 414.2K
 * Testcase Example:  '[[1,2],[3,4]]\n1\n4'
 *
 * In MATLAB, there is a handy function called reshape which can reshape an m x
 * n matrix into a new one with a different size r x c keeping its original
 * data.
 *
 * You are given an m x n matrix mat and two integers r and c representing the
 * number of rows and the number of columns of the wanted reshaped matrix.
 *
 * The reshaped matrix should be filled with all the elements of the original
 * matrix in the same row-traversing order as they were.
 *
 * If the reshape operation with given parameters is possible and legal, output
 * the new reshaped matrix; Otherwise, output the original matrix.
 *
 *
 * Example 1:
 *
 *
 * Input: mat = [[1,2],[3,4]], r = 1, c = 4
 * Output: [[1,2,3,4]]
 *
 *
 * Example 2:
 *
 *
 * Input: mat = [[1,2],[3,4]], r = 2, c = 4
 * Output: [[1,2],[3,4]]
 *
 *
 *
 * Constraints:
 *
 *
 * m == mat.length
 * n == mat[i].length
 * 1 <= m, n <= 100
 * -1000 <= mat[i][j] <= 1000
 * 1 <= r, c <= 300
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let r = r as usize;
    let c = c as usize;
    if r * c != m * n {
      return mat;
    }
    let mut res = vec![vec![0; c]; r];
    for i in 0..r * c {
      res[i / c][i % c] = mat[i / n][i % n];
    }
    res
  }
}
// @lc code=end
