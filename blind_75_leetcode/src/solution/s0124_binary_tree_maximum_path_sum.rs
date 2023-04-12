#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Solution::traversal(&root, &mut max_sum);
        max_sum
    }

    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left_gain = Solution::traversal(&node.borrow().left, max_sum).max(0);
            let right_gain = Solution::traversal(&node.borrow().right, max_sum).max(0);
            *max_sum = (val + left_gain + right_gain).max(*max_sum);

            left_gain.max(right_gain) + val
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests_124 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_path_sum(TreeNode::form_vec(vec![Some(1), Some(2), Some(3)])),
            6
        );
        assert_eq!(
            Solution::max_path_sum(TreeNode::form_vec(vec![
                Some(-10),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            42
        );
    }
}
