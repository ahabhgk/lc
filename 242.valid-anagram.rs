/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 *
 * https://leetcode.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (61.78%)
 * Likes:    5548
 * Dislikes: 231
 * Total Accepted:    1.4M
 * Total Submissions: 2.3M
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * Given two strings s and t, return true if t is an anagram of s, and false
 * otherwise.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, t.length <= 5 * 10^4
 * s and t consist of lowercase English letters.
 *
 *
 *
 * Follow up: What if the inputs contain Unicode characters? How would you
 * adapt your solution to such a case?
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    let mut counter = [0; 26];
    for c in s.chars() {
      counter[c as usize - 'a' as usize] += 1;
    }
    for c in t.chars() {
      counter[c as usize - 'a' as usize] -= 1;
    }
    counter.iter().all(|n| n == &0)
  }
}
// @lc code=end
