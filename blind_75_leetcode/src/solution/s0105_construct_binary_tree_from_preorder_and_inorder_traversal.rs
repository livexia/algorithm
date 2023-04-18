#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_node(&mut 0, &preorder, &inorder, 0, inorder.len())
    }

    fn build_node(
        pre_index: &mut usize,
        preorder: &[i32],
        inorder: &[i32],
        in_start: usize,
        in_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start == in_end {
            None
        } else {
            let root = preorder[*pre_index];
            *pre_index += 1;
            let mut node = TreeNode::new(root);
            let in_index = in_start
                + inorder[in_start..in_end]
                    .iter()
                    .position(|i| i == &root)
                    .unwrap();
            node.left = Solution::build_node(pre_index, preorder, inorder, in_start, in_index);
            node.right = Solution::build_node(pre_index, preorder, inorder, in_index + 1, in_end);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

struct SolutionBFS {}
impl SolutionBFS {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack = vec![root.clone()];
        let mut in_index = 0;
        for &child_val in &preorder[1..] {
            let mut node = stack.last_mut().unwrap().clone();
            let child = Rc::new(RefCell::new(TreeNode::new(child_val)));
            if node.borrow().val != inorder[in_index] {
                node.borrow_mut().left = Some(child.clone());
                stack.push(child);
            } else {
                while !stack.is_empty() && stack.last().unwrap().borrow().val == inorder[in_index] {
                    node = stack.pop().unwrap();
                    in_index += 1;
                }
                node.borrow_mut().right = Some(child.clone());
                stack.push(child);
            }
        }
        Some(root)
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

    #[test]
    fn bfs_works() {
        assert_eq!(
            SolutionBFS::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
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
            SolutionBFS::build_tree(vec![3, 9, 15, 7, 20], vec![15, 9, 7, 3, 20]),
            TreeNode::from_vec(vec![Some(3), Some(9), Some(20), Some(15), Some(7)])
        );
        assert_eq!(
            SolutionBFS::build_tree(vec![-1], vec![-1]),
            TreeNode::from_vec(vec![Some(-1)])
        );
        assert_eq!(
            SolutionBFS::build_tree(vec![1, 2], vec![1, 2]),
            TreeNode::from_vec(vec![Some(1), None, Some(2)])
        );
    }
}
