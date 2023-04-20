#![allow(dead_code)]
pub struct Solution {}

use crate::solution::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
            if let Some(node) = root {
                let l = inorder(&node.borrow().left, k);
                if l.is_some() {
                    return l;
                }
                *k -= 1;
                if *k == 0 {
                    return Some(node.borrow().val);
                }
                let r = inorder(&node.borrow().right, k);
                if r.is_some() {
                    return r;
                }
            }
            None
        }
        inorder(&root, &mut k.clone()).unwrap()
    }
}

#[cfg(test)]
mod tests_230 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::kth_smallest(
                TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]),
                1
            ),
            1
        );
        assert_eq!(
            Solution::kth_smallest(
                TreeNode::from_vec(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(1)
                ]),
                3
            ),
            3
        );
    }
}
