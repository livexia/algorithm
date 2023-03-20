#![allow(dead_code)]

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        for i in v {
            let next = Box::new(ListNode::new(i));
            if let Some(inner_cur) = cur {
                inner_cur.next = Some(next);
                cur = &mut inner_cur.next;
            } else {
                *cur = Some(next)
            }
        }
        head
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut cur = self;
        let mut v = vec![];
        while let Some(next) = &cur.next {
            v.push(cur.val);
            cur = next;
        }
        v.push(cur.val);
        v
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut cur = &mut head;

        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            let l = if l1.val <= l2.val {
                &mut list1
            } else {
                &mut list2
            };
            cur.next = l.take();
            cur = cur.next.as_mut().unwrap();
            *l = cur.next.take();
        }

        cur.next = list1.or(list2);
        head.next
    }
}

impl Solution {
    pub fn merge_two_lists_recursion(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (l1, None) => l1,
            (None, l2) => l2,
            (Some(mut n1), Some(mut n2)) => {
                if n1.val <= n2.val {
                    n1.next = Solution::merge_two_lists_recursion(n1.next, Some(n2));
                    Some(n1)
                } else {
                    n2.next = Solution::merge_two_lists_recursion(Some(n1), n2.next);
                    Some(n2)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_21 {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = ListNode::from_vec(vec![1, 2, 4]);
        let l2 = ListNode::from_vec(vec![1, 3, 4]);
        assert_eq!(
            Solution::merge_two_lists(l1, l2).unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4]
        );

        let l1 = ListNode::from_vec(vec![1, 2, 4]);
        let l2 = ListNode::from_vec(vec![1, 3, 4]);
        assert_eq!(
            Solution::merge_two_lists_recursion(l1, l2)
                .unwrap()
                .to_vec(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
