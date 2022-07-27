/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 *
 * https://leetcode.com/problems/remove-linked-list-elements/description/
 *
 * algorithms
 * Easy (43.96%)
 * Likes:    5373
 * Dislikes: 172
 * Total Accepted:    747.4K
 * Total Submissions: 1.7M
 * Testcase Example:  '[1,2,6,3,4,5,6]\n6'
 *
 * Given the head of a linked list and an integer val, remove all the nodes of
 * the linked list that has Node.val == val, and return the new head.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [1,2,6,3,4,5,6], val = 6
 * Output: [1,2,3,4,5]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [], val = 1
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: head = [7,7,7,7], val = 7
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is in the range [0, 10^4].
 * 1 <= Node.val <= 50
 * 0 <= val <= 50
 *
 *
 */

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

// @lc code=start
impl Solution {
  pub fn remove_elements(
    head: Option<Box<ListNode>>,
    val: i32,
  ) -> Option<Box<ListNode>> {
    match head {
      None => None,
      Some(mut node) => {
        if node.val == val {
          Self::remove_elements(node.next, val)
        } else {
          node.next = Self::remove_elements(node.next, val);
          Some(node)
        }
      }
    }
  }
}
// @lc code=end
