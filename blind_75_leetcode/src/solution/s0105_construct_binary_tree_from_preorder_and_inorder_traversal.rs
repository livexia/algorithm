#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_node(0, &preorder, &inorder, &mut HashSet::new()).1
    }

    fn build_node(
        mut pre_index: usize,
        preorder: &[i32],
        inorder: &[i32],
        visited: &mut HashSet<i32>,
    ) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        if pre_index > preorder.len() - 1 || visited.len() == preorder.len() {
            return (pre_index, None);
        }
        let root = preorder[pre_index];
        let mut node = TreeNode::new(root);
        visited.insert(root);
        let in_index = inorder.iter().position(|&i| i == root).unwrap();
        if in_index == 0 || visited.contains(&inorder[in_index - 1]) {
            pre_index += 1;
            (pre_index, node.right) = if pre_index < preorder.len() {
                Solution::build_node(pre_index, preorder, inorder, visited)
            } else {
                (pre_index, None)
            };
        } else {
            (pre_index, node.left) = if pre_index + 1 < preorder.len() {
                Solution::build_node(pre_index + 1, preorder, inorder, visited)
            } else {
                (pre_index, None)
            };
            (pre_index, node.right) = if pre_index < preorder.len() {
                Solution::build_node(pre_index, preorder, inorder, visited)
            } else {
                (pre_index, None)
            };

            println!("{root} left: {:?}", node.left);
        }
        return (pre_index, Some(Rc::new(RefCell::new(node))));
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
            Solution::build_tree(vec![3, 9, 15, 7, 20], vec![15, 9, 7, 3, 20]),
            TreeNode::from_vec(vec![Some(3), Some(9), Some(20), Some(15), Some(7)])
        );
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            TreeNode::from_vec(vec![Some(-1)])
        );
    }
}
