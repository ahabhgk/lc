/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 *
 * https://leetcode.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (71.03%)
 * Likes:    13430
 * Dislikes: 231
 * Total Accepted:    2.3M
 * Total Submissions: 3.3M
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * Given the head of a singly linked list, reverse the list, and return the
 * reversed list.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [1,2,3,4,5]
 * Output: [5,4,3,2,1]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [1,2]
 * Output: [2,1]
 *
 *
 * Example 3:
 *
 *
 * Input: head = []
 * Output: []
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is the range [0, 5000].
 * -5000 <= Node.val <= 5000
 *
 *
 *
 * Follow up: A linked list can be reversed either iteratively or recursively.
 * Could you implement both?
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
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Self::reverse(&Self, head, None)
  }

  fn reverse(
    &self,
    curr: Option<Box<ListNode>>,
    prev: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    if let Some(mut curr) = curr {
      let next = curr.next;
      curr.next = prev;
      return self.reverse(next, Some(curr));
    }
    prev
  }
}
// @lc code=end
