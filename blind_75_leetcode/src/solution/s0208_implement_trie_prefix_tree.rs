#![allow(dead_code)]

use std::collections::HashMap;

struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Self {
            is_word: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for char in word.chars() {
            node = node.children.entry(char).or_insert(Trie::new());
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for char in word.chars() {
            if let Some(child) = node.children.get(&char) {
                node = child;
            } else {
                return false;
            }
        }
        node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for char in prefix.chars() {
            if let Some(child) = node.children.get(&char) {
                node = child;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests_208 {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
