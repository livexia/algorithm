#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    // This is a template
}

#[derive(Default)]
struct WordDictionary {
    is_word: bool,
    children: [Option<Box<WordDictionary>>; 26],
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for index in word.bytes().map(|b| (b - b'a') as usize) {
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(WordDictionary::new()));
            }
            node = node.children[index].as_mut().unwrap();
        }
        node.is_word = true;
    }

    fn _search(&self, bytes: &[usize], index: usize) -> bool {
        if index >= bytes.len() {
            self.is_word
        } else if bytes[index] == 27 {
            self.children
                .iter()
                .filter_map(|c| c.as_ref())
                .any(|c| c._search(bytes, index + 1))
        } else if let Some(node) = &self.children[bytes[index]] {
            node._search(bytes, index + 1)
        } else {
            false
        }
    }

    fn search(&self, word: String) -> bool {
        let bytes: Vec<_> = word
            .bytes()
            .map(|b| if b == b'.' { 27 } else { (b - b'a') as usize })
            .collect();
        self._search(&bytes, 0)
    }
}

#[cfg(test)]
mod tests_211 {
    use super::*;

    #[test]
    fn it_works() {
        let mut w = WordDictionary::new();
        w.add_word("bad".to_string());
        w.add_word("dad".to_string());
        w.add_word("mad".to_string());
        assert_eq!(w.search("b".to_string()), false);
        assert_eq!(w.search(".bad".to_string()), false);
        assert_eq!(w.search("pad".to_string()), false);
        assert_eq!(w.search("bad".to_string()), true);
        assert_eq!(w.search(".ad".to_string()), true);
        assert_eq!(w.search("b..".to_string()), true);
    }
}
