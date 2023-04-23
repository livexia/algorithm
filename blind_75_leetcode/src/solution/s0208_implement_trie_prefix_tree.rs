#![allow(dead_code)]

#[derive(Default)]
struct Trie {
    is_word: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for index in word.bytes().map(|b| (b - b'a') as usize) {
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(Trie::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }
        node.is_word = true;
    }

    fn search_prefix(&self, prefix: &str) -> Option<bool> {
        let mut node = self;
        for index in prefix.bytes().map(|b| (b - b'a') as usize) {
            if let Some(child) = &node.children[index] {
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
