#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::{search_val, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests_235 {
    use super::*;

    #[test]
    fn it_works() {
        let root = TreeNode::from_vec(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        assert_eq!(
            Solution::lowest_common_ancestor(
                root.clone(),
                search_val(&root, 2),
                search_val(&root, 8)
            ),
            search_val(&root, 6)
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                root.clone(),
                search_val(&root, 2),
                search_val(&root, 4)
            ),
            search_val(&root, 2)
        );
    }
}
