use std::error::Error;
use std::fmt::Debug;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct BinarySearchTree<T> {
    data: T,
    size: usize,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
    // parent: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    fn new(data: T) -> Self {
        BinarySearchTree {
            data,
            size: 1,
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
                self.left = Some(Box::new(BinarySearchTree::new(data)));
            } else {
                self.left.as_mut().unwrap().insert(data)?;
            }
        }
        if data > self.data {
            if self.right.is_none() {
                self.right = Some(Box::new(BinarySearchTree::new(data)));
            } else {
                self.right.as_mut().unwrap().insert(data)?;
            }
        }
        self.size += 1;
        Ok(())
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

    fn rank(&self, t: T) -> usize {
        let mut count = 0;
        let mut cur = Some(self);
        while let Some(node) = cur {
            if node.data <= t {
                count += 1;
                count += if let Some(left) = &node.left {
                    left.size
                } else {
                    0
                };
                cur = node.right.as_deref();
            } else {
                cur = node.left.as_deref();
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::BinarySearchTree;
    use std::error::Error;
    use std::result;

    type Result<T> = result::Result<T, Box<dyn Error>>;

    #[test]
    fn it_works() -> Result<()> {
        let mut tree = BinarySearchTree::new(60);
        tree.insert(80)?;
        tree.insert(40)?;
        assert_eq!(tree.find_min(), 40);
        tree.insert(99)?;
        tree.insert(33)?;
        tree.insert(45)?;
        assert_eq!(tree.rank(80), 5);
        assert!(tree.find(99));
        assert!(tree.find(60));
        assert!(tree.find(33));
        assert_eq!(tree.find_min(), 33);
        Ok(())
    }
}
