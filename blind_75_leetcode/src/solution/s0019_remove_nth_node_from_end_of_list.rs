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
        let mut count = 0;
        let mut cur = head.as_ref();
        while let Some(inner) = cur {
            cur = inner.next.as_ref();
            count += 1;
        }
        // we can sure that the length of linked list is count
        // so use unwrap is fine
        if count == n {
            return head.as_mut().unwrap().next.take();
        }
        let mut cur = head.as_mut().unwrap();
        for _ in 0..count - n - 1 {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = cur.next.take().unwrap().next;

        head
    }

    pub fn remove_nth_from_end_unsafe(
        mut head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut count = 0;
        unsafe {
            let mut fast = &head as *const Option<Box<ListNode>>;
            let mut slow = &mut head as *mut Option<Box<ListNode>>;
            while (*fast).is_some() {
                fast = &(*fast).as_ref().unwrap().next as *const Option<Box<ListNode>>;
                if count >= n {
                    slow = &mut (*slow).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                }
                count += 1;
            }
            *slow = (*slow).as_mut().unwrap().next.take();
        }
        head
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
            vec![1]
        );
        let list = ListNode::from_vec(vec![1, 2]);
        assert_eq!(
            Solution::remove_nth_from_end(list, 2).unwrap().to_vec(),
            vec![2]
        );
    }

    #[test]
    fn it_works_unsafe() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            Solution::remove_nth_from_end_unsafe(list, 2)
                .unwrap()
                .to_vec(),
            vec![1, 2, 3, 5]
        );
        let list = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::remove_nth_from_end_unsafe(list, 1), None);
        let list = ListNode::from_vec(vec![1, 2]);
        assert_eq!(
            Solution::remove_nth_from_end_unsafe(list, 1)
                .unwrap()
                .to_vec(),
            vec![1]
        );
        let list = ListNode::from_vec(vec![1, 2]);
        assert_eq!(
            Solution::remove_nth_from_end_unsafe(list, 2)
                .unwrap()
                .to_vec(),
            vec![2]
        );
    }
}
