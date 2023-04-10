#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Solution::invert_tree(right);
            node.borrow_mut().right = Solution::invert_tree(left);
            Some(node)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests_226 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::invert_tree(TreeNode::form_vec(
                [4, 2, 7, 1, 3, 6, 9].into_iter().map(|i| Some(i)).collect()
            )),
            TreeNode::form_vec([4, 7, 2, 9, 6, 3, 1].into_iter().map(|i| Some(i)).collect())
        );
    }
}
