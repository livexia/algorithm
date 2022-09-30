#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn clone_graph(node: Rc<Node>) -> Rc<Node> {
        let node = Rc::new(node);
        let mut visited = HashMap::new();
        Solution::dfs(&node, &mut visited)
    }

    fn dfs(node: &Rc<Node>, visited: &mut HashMap<i32, Rc<Node>>) -> Rc<Node> {
        if let Some(node) = visited.get(&node.val) {
            return node.clone();
        }
        let val = node.val.clone();
        let neighbors = node
            .neighbors
            .iter()
            .map(|n| Solution::dfs(n, visited))
            .collect();
        let node = Rc::new(Node::new(val, neighbors));
        visited.insert(val, node.clone());
        node
    }
}

pub struct Node {
    val: i32,
    neighbors: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(val: i32, neighbors: Vec<Rc<Node>>) -> Self {
        Node { val, neighbors }
    }
}

#[cfg(test)]
mod tests_133 {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, 0);
    }
}
