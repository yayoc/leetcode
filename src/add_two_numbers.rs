// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example:

// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
pub struct Solution1;

pub trait Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

impl Solution for Solution1 {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let (mut one, mut two, mut carry) = (l1, l2, 0);
        let mut x: i32;
        let mut y: i32;
        let mut current = root.as_mut();
        while one.is_some() || two.is_some() {
            if let Some(v) = one {
                x = v.val;
                one = v.next;
            } else {
                x = 0;
            }
            if let Some(v) = two {
                y = v.val;
                two = v.next;
            } else {
                y = 0;
            }
            let sum = carry + x + y;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            unsafe {
                current = (*(*(current as *mut ListNode)).next.as_mut().unwrap()).as_mut();
            }
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }
        root.next
    }
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_two_numbers() {
        assert_eq!(to_list(vec![7, 0, 8]), Solution1::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])));
        assert_eq!(to_list(vec![0]), Solution1::add_two_numbers(to_list(vec![0]), to_list(vec![0])));
    }
}