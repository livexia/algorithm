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
    data: T,
    height: i32,
    left: Option<Box<AVLTree<T>>>,
    right: Option<Box<AVLTree<T>>>,
    // parent: Option<Box<AVLTree<T>>>,
}

impl<T> AVLTree<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new(data: T) -> Self {
        AVLTree {
            data,
            height: 1,
            left: None,
            right: None,
            // parent: None,
        }
    }

    fn insert(&mut self, data: T) -> Result<()> {
        if data == self.data {
            return err!("can not insert {:?} into binary search tree", data);
        }
        if data < self.data {
            if self.left.is_none() {
                self.left = Some(Box::new(AVLTree::new(data)));
            } else {
                self.left.as_mut().unwrap().insert(data)?;
            }
            self.height = self.height.max(self.left.as_ref().unwrap().height + 1)
        }
        if data > self.data {
            if self.right.is_none() {
                self.right = Some(Box::new(AVLTree::new(data)));
            } else {
                self.right.as_mut().unwrap().insert(data)?;
            }
            self.height = self.height.max(self.right.as_ref().unwrap().height + 1)
        }
        Ok(())
    }

    fn left_rotate(&mut self) {
        
    }

    fn right_rotate(&mut self) {
        
    }

    fn is_right_heavy(&self) -> bool {
        let left_height = if let Some(left) = &self.left {
            left.height
        } else {
            0
        };
        let right_height = if let Some(right) = &self.right {
            right.height
        } else {
            0
        };
        right_height > left_height + 1
    }

    fn find_min(&self) -> T {
        let mut min = self.data;
        let mut cur = self;
        while let Some(node) = &cur.left {
            min = node.data;
            cur = &node;
        }
        min
    }

    fn find(&self, data: T) -> bool {
        let mut cur = Some(self);
        while let Some(node) = cur {
            if node.data > data {
                cur = node.left.as_deref();
            } else if node.data < data {
                cur = node.right.as_deref();
            } else {
                return true
            }
        }
        false
    }
}


fn in_order_traversal<T>(node: &Option<Box<AVLTree<T>>>) -> Vec<T> 
where
    T: Clone + Copy
{
    let mut result = vec![];
    if let Some(node) = node {
        result.append(&mut in_order_traversal(&node.left));
        result.push(node.data);
        result.append(&mut in_order_traversal(&node.right));
    } result

}

#[cfg(test)]
mod tests {
    use crate::AVLTree;
    use crate::in_order_traversal;
    use std::error::Error;
    use std::result;

    type Result<T> = result::Result<T, Box<dyn Error>>;

    #[test]
    fn it_works() -> Result<()> {
        let nums: Vec<i32> = (0..3).collect();

        let mut sorted = nums.clone();
        sorted.sort();

        let mut tree = AVLTree::new(nums[0]);
        for &i in &nums[1..] {
            tree.insert(i)?;
        }
        
        println!("{:?}", tree);

        assert_eq!(in_order_traversal(&Some(Box::new(tree))), sorted);
        Ok(())
    }
}

