/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 *
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (49.35%)
 * Likes:    5356
 * Dislikes: 195
 * Total Accepted:    930.8K
 * Total Submissions: 1.9M
 * Testcase Example:  '[1,1,2]'
 *
 * Given the head of a sorted linked list, delete all duplicates such that each
 * element appears only once. Return the linked list sorted as well.
 *
 *
 * Example 1:
 *
 *
 * Input: head = [1,1,2]
 * Output: [1,2]
 *
 *
 * Example 2:
 *
 *
 * Input: head = [1,1,2,3,3]
 * Output: [1,2,3]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in the list is in the range [0, 300].
 * -100 <= Node.val <= 100
 * The list is guaranteed to be sorted in ascending order.
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
  pub fn delete_duplicates(
    head: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    Self::delete(head, None)
  }

  fn delete(
    curr: Option<Box<ListNode>>,
    val: Option<i32>,
  ) -> Option<Box<ListNode>> {
    match curr {
      Some(curr) => {
        if matches!(val, Some(v) if v == curr.val) {
          return Self::delete(curr.next, val);
        }
        Some(Box::new(ListNode {
          val: curr.val,
          next: Self::delete(curr.next, Some(curr.val)),
        }))
      }
      None => None,
    }
  }
}
// @lc code=end
