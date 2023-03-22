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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut temp_head = head.as_ref();
        let mut count = 0;

        let mut cur = head.as_ref();
        while let Some(inner) = cur {
            cur = inner.next.as_ref();
            if count > n {
                if let Some(inner_head) = temp_head {
                    temp_head = inner_head.next.as_ref()
                }
            }
            count += 1;
        }
        println!("{:?}", temp_head);
        println!("{:?}", head);
        if count == n {
            if let Some(inner) = head.take() {
                return inner.next;
            }
        } else if let Some(inner) = temp_head.as_mut() {
            // if let Some(next) = inner.next.take() {
            // inner.next = next.next;
            // } else {
            // return None;
            // }
        }
        None
    }
}

#[cfg(test)]
mod tests_19 {
    use super::*;

    #[test]
    fn it_works() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            Solution::remove_nth_from_end(list, 2).unwrap().to_vec(),
            vec![1, 2, 3, 5]
        );
        let list = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::remove_nth_from_end(list, 1), None);
        let list = ListNode::from_vec(vec![1, 2]);
        assert_eq!(
            Solution::remove_nth_from_end(list, 1).unwrap().to_vec(),
            vec![2]
        );
        let list = ListNode::from_vec(vec![1, 2]);
        assert_eq!(
            Solution::remove_nth_from_end(list, 2).unwrap().to_vec(),
            vec![2]
        );
    }
}
