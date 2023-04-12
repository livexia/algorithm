#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));
        let mut r = vec![];
        while let Some((node, level)) = queue.pop_front() {
            if let Some(node) = node {
                if level == r.len() {
                    r.push(vec![]);
                }
                let v = r.last_mut().unwrap();
                let mut node = node.borrow_mut();
                v.push(node.val);
                queue.push_back((node.left.take(), level + 1));
                queue.push_back((node.right.take(), level + 1));
            }
        }

        r
    }
}

#[cfg(test)]
mod tests_102 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::level_order(TreeNode::form_vec(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
