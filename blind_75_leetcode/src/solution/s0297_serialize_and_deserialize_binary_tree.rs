#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    // This is a template
}

use super::s0104_maximum_depth_of_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut r = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut last = 0;
        while let Some(node) = queue.pop_front() {
            if let Some(node) = node {
                r.push(node.borrow().val.to_string());
                if let Some(node) = &node.borrow().left {
                    queue.push_back(Some(node.clone()));
                } else {
                    queue.push_back(None);
                }
                if let Some(node) = &node.borrow().right {
                    queue.push_back(Some(node.clone()));
                } else {
                    queue.push_back(None);
                }
                last = r.len();
            } else {
                r.push("null".to_string());
            }
        }

        format!("[{}]", r[..last].join(","))
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut data = data
            .trim_matches('[')
            .trim_matches(']')
            .split(',')
            .map(|w| w.trim());
        let mut queue = VecDeque::new();

        let root = if let Some(root_val) = data.next() {
            if let Some(root) = Codec::str_val_to_node(root_val) {
                queue.push_back(root.clone());
                Some(root)
            } else {
                None
            }
        } else {
            None
        };
        while let Some(node) = queue.pop_front() {
            if let Some(left_val) = data.next() {
                if let Some(left) = Codec::str_val_to_node(left_val) {
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
            } else {
                break;
            }
            if let Some(right_val) = data.next() {
                if let Some(right) = Codec::str_val_to_node(right_val) {
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
            }
        }
        root
    }

    fn str_val_to_node(val: &str) -> Option<Rc<RefCell<TreeNode>>> {
        if val == "null" || val.is_empty() {
            None
        } else if let Ok(val) = val.parse::<i32>() {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests_297 {
    use super::*;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let root = TreeNode::form_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);
        let strs = "[1,2,3,null,null,4,5]";
        assert_eq!(&codec.serialize(root.clone()), strs);
        assert_eq!(codec.deserialize(strs.to_string()), root);
        assert_eq!(
            codec.deserialize("[1,2]".to_string()),
            TreeNode::form_vec(vec![Some(1), Some(2)])
        );
    }
}
