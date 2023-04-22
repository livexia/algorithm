#![allow(dead_code)]
pub struct Solution {}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p: &Option<Rc<RefCell<TreeNode>>>,
            q: &Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                if let Ordering::Less = cmp_with_option(root, p) {
                    dfs(&node.borrow().right, p, q)
                } else if let Ordering::Greater = cmp_with_option(root, q) {
                    dfs(&node.borrow().left, p, q)
                } else {
                    root.clone()
                }
            } else {
                None
            }
        }
        let (p, q) = if let Ordering::Greater = cmp_with_option(&p, &q) {
            (q, p)
        } else {
            (p, q)
        };
        dfs(&root, &p, &q)
    }
}

fn cmp_with_option(
    p: &Option<Rc<RefCell<TreeNode>>>,
    q: &Option<Rc<RefCell<TreeNode>>>,
) -> Ordering {
    match (p, q) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(p), Some(q)) => p.borrow().val.cmp(&q.borrow().val),
    }
}

#[cfg(test)]
mod tests_235 {
    use super::super::s0104_maximum_depth_of_binary_tree::search_val;
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
