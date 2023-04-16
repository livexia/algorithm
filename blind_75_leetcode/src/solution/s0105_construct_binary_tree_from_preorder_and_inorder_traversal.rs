#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}
#[cfg(test)]
mod tests_105 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            TreeNode::from_vec(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            TreeNode::from_vec(vec![Some(-1)])
        );
    }
}
