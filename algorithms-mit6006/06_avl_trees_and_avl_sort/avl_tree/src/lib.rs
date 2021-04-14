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
    // parent: Option<Box<AVLTree<T>>>,
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
            // parent: None,
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
            self.height = self.height.max(self.left.as_ref().unwrap().height + 1)
        }
        if data > self.data {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(data)));
            } else {
                self.right = self.right.take().unwrap().insert(data)?
            }
            self.height = self.height.max(self.right.as_ref().unwrap().height + 1)
        }
        Ok(Some(Box::new(self)))
    }

    fn left_rotate(&mut self) {}

    fn right_rotate(&mut self) {}

    fn is_right_heavy(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::AVLTree;
    use std::error::Error;
    use std::result;

    type Result<T> = result::Result<T, Box<dyn Error>>;

    #[test]
    fn it_works() -> Result<()> {
        let nums: Vec<i32> = (0..4).collect();

        let mut sorted = nums.clone();
        sorted.sort();

        let mut tree = AVLTree::new();
        for &i in &nums {
            tree.insert(i)?;
        }

        println!("{:?}", tree);

        assert_eq!(tree.in_order_traversal(), sorted);
        Ok(())
    }
}
