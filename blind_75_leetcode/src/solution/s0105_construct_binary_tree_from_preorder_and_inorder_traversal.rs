#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_index = 0;
        let mut in_index = 0;
        let mut in_visited = std::collections::HashSet::new();
        while in_visited.len() != preorder.len() {
            let mut root = preorder[pre_index];
            in_visited.insert(root);
            println!("root: {root}");
            pre_index += 1;
            let left = preorder[pre_index];
            in_visited.insert(left);
            println!("left: {left}");
            let in_index = inorder.iter().position(|&i| i == left).unwrap();
            if in_index == 0 || in_visited.contains(&inorder[in_index - 1]) {
                pre_index += 1;
                let right = preorder[pre_index];
                in_visited.insert(right);
                println!("right: {right}")
            } else {
                println!("++++");
            }
        }

        todo!()
    }
}
#[cfg(test)]
mod tests_105 {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
        //     TreeNode::from_vec(vec![
        //         Some(3),
        //         Some(9),
        //         Some(20),
        //         None,
        //         None,
        //         Some(15),
        //         Some(7)
        //     ])
        // );
        assert_eq!(
            Solution::build_tree(vec![3, 9, 15, 7, 20], vec![15, 9, 7, 3, 20]),
            TreeNode::from_vec(vec![Some(3), Some(9), Some(20), Some(15), Some(7)])
        );
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            TreeNode::from_vec(vec![Some(-1)])
        );
    }
}
