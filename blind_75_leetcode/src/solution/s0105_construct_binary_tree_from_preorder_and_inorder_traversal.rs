#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_node(&preorder, &inorder)
    }

    fn build_node(
        preorder: &[i32],
        inorder: &[i32],
        // preorder_index: &HashMap<i32, usize>,
        // inorder_index: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 1 {
            Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))))
        } else if preorder.is_empty() {
            None
        } else {
            let preorder_index: HashMap<i32, usize> =
                preorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            let inorder_index: HashMap<i32, usize> =
                inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            let root = preorder[0];
            let mut node = TreeNode::new(root);
            let &in_index = inorder_index.get(&root).unwrap();
            let left = if let Some(&left) = inorder[..in_index]
                .iter()
                .map(|i| preorder_index.get(i).unwrap())
                .max()
            {
                node.left = Solution::build_node(&preorder[1..=left], &inorder[..in_index]);
                left
            } else {
                node.left = None;
                0
            };
            node.right = Solution::build_node(&preorder[left + 1..], &inorder[in_index + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
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
        assert_eq!(
            Solution::build_tree(vec![1, 2], vec![1, 2]),
            TreeNode::from_vec(vec![Some(1), None, Some(2)])
        );
    }
}
