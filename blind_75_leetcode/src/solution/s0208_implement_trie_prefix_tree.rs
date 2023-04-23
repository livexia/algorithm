#![allow(dead_code)]

use std::collections::HashMap;

struct Trie {
    is_word: bool,
    children: HashMap<u8, Trie>,
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
        for byte in word.bytes() {
            node = node.children.entry(byte).or_insert(Trie::new());
        }
        node.is_word = true;
    }

    fn search_prefix(&self, prefix: &str) -> Option<bool> {
        let mut node = self;
        for byte in prefix.bytes() {
            if let Some(child) = node.children.get(&byte) {
                node = child;
            } else {
                return None;
            }
        }
        Some(node.is_word)
    }

    fn search(&self, word: String) -> bool {
        self.search_prefix(&word).unwrap_or(false)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.search_prefix(&prefix).is_some()
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
