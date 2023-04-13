#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn form_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        // level order
        if v.is_empty() {
            return None;
        }
        let head = if let Some(val) = v[0] {
            Rc::new(RefCell::new(TreeNode::new(val)))
        } else {
            return None;
        };
        let mut queue: VecDeque<_> = VecDeque::new();
        queue.push_back(head.clone());

        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            if let Some(&Some(val)) = v.get(i) {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            if let Some(&Some(val)) = v.get(i + 1) {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 2;
        }
        Some(head)
    }

    pub fn level_order(&self) -> Vec<i32> {
        let mut r = vec![self.val];
        let mut queue = VecDeque::new();
        if let Some(left) = &self.left {
            queue.push_back(left.clone());
        }
        if let Some(right) = &self.right {
            queue.push_back(right.clone());
        }
        while let Some(node) = queue.pop_front() {
            r.push(node.borrow().val);
            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
            };
            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
            };
        }

        r
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back((node, 1));
        } else {
            return 0;
        }
        let mut count = 0;
        while let Some((node, level)) = queue.pop_front() {
            count = level;
            if let Some(left) = node.borrow_mut().left.take() {
                queue.push_back((left, level + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push_back((right, level + 1));
            }
        }

        count
    }
}

pub struct Solution2 {}

impl Solution2 {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let hl = Solution2::max_depth(node.borrow_mut().left.take());
            let hr = Solution2::max_depth(node.borrow_mut().right.take());
            hl.max(hr) + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests_104 {
    use super::*;

    #[test]
    fn it_works() {
        let root = TreeNode::form_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn create_tree() {
        let root = TreeNode::form_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(root.unwrap().borrow().level_order(), vec![3, 9, 20, 15, 7]);
    }
}
