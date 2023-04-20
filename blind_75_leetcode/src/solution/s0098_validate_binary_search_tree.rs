#![allow(dead_code)]
pub struct Solution {}

use crate::solution::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::validation_bst(&root, None, None)
    }

    fn validation_bst(
        node: &Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        if let Some(node) = node {
            let node = node.borrow();
            let (mut l_flag, mut r_flag) = (true, true);
            if let Some(min) = min {
                l_flag = node.val > min;
            }
            if let Some(max) = max {
                r_flag = node.val < max;
            }
            l_flag
                && r_flag
                && Solution::validation_bst(&node.left, min, Some(node.val))
                && Solution::validation_bst(&node.right, Some(node.val), max)
        } else {
            true
        }
    }
}

pub struct Solution2 {}
impl Solution2 {
    pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut inorder = None;

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root.take() {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }

            let node = stack.pop().unwrap();
            let val = node.borrow().val;
            if let Some(inorder) = inorder {
                if val <= inorder {
                    return false;
                }
            }
            inorder = Some(val);
            root = node.borrow_mut().right.take();
        }
        true
    }
}

#[cfg(test)]
mod tests_98 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2), Some(1), Some(3)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2147483647)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(4),
                Some(6),
                None,
                None,
                Some(3),
                Some(7)
            ])),
            false
        );
    }

    #[test]
    fn it_works_stack_inorder() {
        use super::Solution2 as Solution;

        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2), Some(1), Some(3)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![Some(2147483647)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![
                Some(5),
                Some(4),
                Some(6),
                None,
                None,
                Some(3),
                Some(7)
            ])),
            false
        );
    }
}
