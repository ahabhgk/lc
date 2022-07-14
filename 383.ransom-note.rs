/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 *
 * https://leetcode.com/problems/ransom-note/description/
 *
 * algorithms
 * Easy (56.16%)
 * Likes:    1983
 * Dislikes: 327
 * Total Accepted:    436.5K
 * Total Submissions: 776.4K
 * Testcase Example:  '"a"\n"b"'
 *
 * Given two strings ransomNote and magazine, return true if ransomNote can be
 * constructed by using the letters from magazine and false otherwise.
 *
 * Each letter in magazine can only be used once in ransomNote.
 *
 *
 * Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *
 *
 * Constraints:
 *
 *
 * 1 <= ransomNote.length, magazine.length <= 10^5
 * ransomNote and magazine consist of lowercase English letters.
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
  pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut counter = [0; 26];
    for c in magazine.chars() {
      counter[c as usize - 'a' as usize] += 1;
    }
    for c in ransom_note.chars() {
      let i = c as usize - 'a' as usize;
      counter[i] -= 1;
      if counter[i] < 0 {
        return false;
      }
    }
    true
  }
}
// @lc code=end

#[test]
fn smock() {
  assert_eq!(
    Solution::can_construct("aa".to_owned(), "aab".to_owned()),
    true
  );
  assert_eq!(
    Solution::can_construct(
      "bg".to_owned(),
      "efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj".to_owned()
    ),
    true
  );
}
