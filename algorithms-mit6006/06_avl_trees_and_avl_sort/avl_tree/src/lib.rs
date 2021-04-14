/**
AVL Trees
- require height of left and right children of every node differ by at most 1
- h < 1.44*log2(n)
- Rotation
    - Left-Rotate(x)
- AVL insert
    - simple BST insert
    - fix AVL property
        - suppose x id the lowest node violating AVL
        - assume x.right higher
        - if x.right child is right-heavy or balanced
            - Left-Rotate(x)
        - else if x'right child is left-heavy or balanced
            - Right-Rotate(x.right)
            - Left-Rotate(x)
- AVL Sort
    - Insert n items -> Θ(nlogn)
    - in-order traversal -> Θ(n)
*/
use std::error::Error;
use std::fmt::Debug;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> AVLTree<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, data: T) -> Result<()> {
        if self.root.is_some() {
            self.root = self.root.take().unwrap().insert(data)?;
        } else {
            self.root = Some(Box::new(Node::new(data)))
        }
        Ok(())
    }

    fn find_min(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut min = self.root.as_ref().unwrap().data;
        let mut cur = self.root.as_ref().unwrap();
        while let Some(node) = &cur.left {
            min = node.data;
            cur = &node;
        }
        Some(min)
    }

    fn find(&self, data: T) -> bool {
        if self.is_empty() {
            return false;
        }
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            if node.data > data {
                cur = node.left.as_deref();
            } else if node.data < data {
                cur = node.right.as_deref();
            } else {
                return true;
            }
        }
        false
    }

    pub fn in_order_traversal(&self) -> Vec<T> {
        if self.is_empty() {
            vec![]
        } else {
            AVLTree::traversal(&self.root)
        }
    }

    fn traversal(node: &Option<Box<Node<T>>>) -> Vec<T> {
        let mut result = vec![];
        if let Some(node) = node {
            result.append(&mut AVLTree::traversal(&node.left));
            result.push(node.data);
            result.append(&mut AVLTree::traversal(&node.right));
        }
        result
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new(data: T) -> Self {
        Node {
            data,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn insert(mut self, data: T) -> Result<Option<Box<Self>>> {
        if data == self.data {
            return err!("can not insert {:?} into binary search tree", data);
        }
        if data < self.data {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::new(data)));
            } else {
                self.left = self.left.take().unwrap().insert(data)?
            }
        }
        if data > self.data {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(data)));
            } else {
                self.right = self.right.take().unwrap().insert(data)?
            }
        }
        self.update_height();
        if !self.is_avl() {
            self = self.avlify();
        }
        Ok(Some(Box::new(self)))
    }

    fn avlify(mut self) -> Node<T> {
        if self.is_right_heavy() {
            let right = self.right.as_ref().unwrap();
            if Node::get_height(&right.right) > Node::get_height(&right.left) || right.is_balanced()
            {
                self = self.left_rotate();
            } else {
                let right = self.right.take();
                self.right = Some(Box::new(right.unwrap().right_rotate()));
                self = self.left_rotate();
            }
        }
        if self.is_left_heavy() {
            let left = self.left.as_ref().unwrap();
            if Node::get_height(&left.left) > Node::get_height(&left.right) || left.is_balanced() {
                self = self.right_rotate();
            } else {
                let left = self.left.take();
                self.left = Some(Box::new(left.unwrap().left_rotate()));
                self = self.right_rotate();
            }
        }
        self
    }

    fn left_rotate(mut self) -> Node<T> {
        let mut right = self.right.take().unwrap();
        let right_left = right.left.take();
        self.right = right_left;
        self.update_height();
        right.left = Some(Box::new(self));
        right.update_height();
        *right
    }

    fn right_rotate(mut self) -> Node<T> {
        let mut left = self.left.take().unwrap();
        let left_right = left.right.take();
        self.left = left_right;
        self.update_height();
        left.right = Some(Box::new(self));
        left.update_height();
        *left
    }

    fn is_right_heavy(&self) -> bool {
        let left_height = Node::get_height(&self.left);
        let right_height = Node::get_height(&self.right);
        right_height > left_height
    }

    fn is_left_heavy(&self) -> bool {
        let left_height = Node::get_height(&self.left);
        let right_height = Node::get_height(&self.right);
        left_height > right_height
    }

    fn is_balanced(&self) -> bool {
        let left_height = Node::get_height(&self.left);
        let right_height = Node::get_height(&self.right);
        left_height == right_height
    }

    fn is_avl(&self) -> bool {
        let left_height = Node::get_height(&self.left);
        let right_height = Node::get_height(&self.right);
        (left_height - right_height).abs() < 2
    }

    fn get_height(node: &Option<Box<Node<T>>>) -> i32 {
        if node.is_none() {
            0
        } else {
            node.as_ref().unwrap().height
        }
    }

    fn update_height(&mut self) {
        let left_height = Node::get_height(&self.left);
        let right_height = Node::get_height(&self.right);
        self.height = left_height.max(right_height) + 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::AVLTree;
    use crate::Node;
    use rand::{thread_rng, Rng};
    use std::error::Error;
    use std::fmt::Debug;
    use std::result;

    type Result<T> = result::Result<T, Box<dyn Error>>;

    fn is_avl_tree<T>(node: &Node<T>) -> bool
    where
        T: Clone + Copy + PartialEq + Eq + Ord + Debug,
    {
        if node.left.is_some() {
            if !is_avl_tree(node.left.as_ref().unwrap()) {
                return false;
            }
        }
        if node.right.is_some() {
            if !is_avl_tree(node.right.as_ref().unwrap()) {
                return false;
            }
        }
        node.is_avl()
    }

    #[test]
    fn test1() -> Result<()> {
        let nums: Vec<i32> = (0..4).collect();

        let mut sorted = nums.clone();
        sorted.sort();

        let mut tree = AVLTree::new();
        for &i in &nums {
            tree.insert(i)?;
        }

        assert!(is_avl_tree(tree.root.as_ref().unwrap()));
        assert_eq!(tree.in_order_traversal(), sorted);
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let nums: Vec<i32> = (0..4).rev().collect();

        let mut sorted = nums.clone();
        sorted.sort();

        let mut tree = AVLTree::new();
        for &i in &nums {
            tree.insert(i)?;
        }

        assert!(is_avl_tree(tree.root.as_ref().unwrap()));
        assert_eq!(tree.in_order_traversal(), sorted);
        Ok(())
    }

    #[test]
    fn test3() -> Result<()> {
        let nums = [
            1817101522,
            1179914281,
            -285167116,
            -748156548,
            1602561484,
            1115647986,
            -756677515,
            700230789,
            827992271,
            -1353954074,
        ];
        let nums = nums.to_vec();

        let mut sorted = nums.clone();
        sorted.sort();

        let mut tree = AVLTree::new();
        for &i in &nums {
            tree.insert(i)?;
        }
        assert!(is_avl_tree(tree.root.as_ref().unwrap()));
        assert_eq!(tree.in_order_traversal(), sorted);
        Ok(())
    }

    #[test]
    fn rand_test() -> Result<()> {
        for _ in 0..100 {
            let mut nums = [0i32; 1000];
            thread_rng().fill(&mut nums[..]);
            let nums = nums.to_vec();

            let mut sorted = nums.clone();
            sorted.sort();

            let mut tree = AVLTree::new();
            for &i in &nums {
                tree.insert(i)?;
            }
            assert!(is_avl_tree(tree.root.as_ref().unwrap()));
            assert_eq!(tree.in_order_traversal(), sorted);
        }
        Ok(())
    }
}
