/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 *
 * https://leetcode.com/problems/merge-two-sorted-lists/description/
 *
 * algorithms
 * Easy (60.88%)
 * Likes:    13363
 * Dislikes: 1209
 * Total Accepted:    2.4M
 * Total Submissions: 3.9M
 * Testcase Example:  '[1,2,4]\n[1,3,4]'
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 *
 * Merge the two lists in a one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 *
 * Return the head of the merged linked list.
 *
 *
 * Example 1:
 *
 *
 * Input: list1 = [1,2,4], list2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 *
 * Example 2:
 *
 *
 * Input: list1 = [], list2 = []
 * Output: []
 *
 *
 * Example 3:
 *
 *
 * Input: list1 = [], list2 = [0]
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * The number of nodes in both lists is in the range [0, 50].
 * -100 <= Node.val <= 100
 * Both list1 and list2 are sorted in non-decreasing order.
 *
 *
 */

pub struct Solution;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

// @lc code=start
impl Solution {
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    match (list1, list2) {
      (None, None) => None,
      (Some(n), None) | (None, Some(n)) => Some(n),
      (Some(l1), Some(l2)) => {
        if l1.val < l2.val {
          Some(Box::new(ListNode {
            val: l1.val,
            next: Solution::merge_two_lists(l1.next, Some(l2)),
          }))
        } else {
          Some(Box::new(ListNode {
            val: l2.val,
            next: Solution::merge_two_lists(Some(l1), l2.next),
          }))
        }
      }
    }
  }
}
// @lc code=end
