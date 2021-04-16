pub mod dnaseqlib;

use dnaseqlib::RollingHash;
use std::{collections::HashMap, slice::Iter};

#[derive(Debug)]
pub struct Multidict<T> {
    data: HashMap<u64, Vec<T>>,
}

impl<T: Clone + Default> Multidict<T> {
    pub fn new(pairs: Vec<(u64, T)>) -> Self {
        let mut data: HashMap<u64, Vec<T>> = HashMap::new();
        for (hash, v) in pairs.clone() {
            data.entry(hash).or_default().push(v)
        }
        Self { data }
    }

    pub fn put(&mut self, k: u64, v: T) {
        self.data.entry(k).or_default().push(v)
    }

    pub fn get(&self, k: &u64) -> Vec<T> {
        self.data.get(k).unwrap_or(&Vec::new()).to_vec()
    }
}

pub struct SubSequenceHashIterator<'a> {
    seq: &'a mut Iter<'a, u8>,
    rh: dnaseqlib::RollingHash,
    sub: Vec<u8>,
    pos: usize,
}

impl<'a> SubSequenceHashIterator<'a> {
    pub fn new(seq: &'a mut Iter<'a, u8>, length: usize) -> Self {
        let mut sub = vec![];
        for _ in 0..length {
            sub.push(seq.next().unwrap().clone())
        }
        let rh = RollingHash::new(&sub[..]);
        Self {
            seq,
            rh,
            sub,
            pos: 0,
        }
    }
}

impl Iterator for SubSequenceHashIterator<'_> {
    type Item = (u64, (usize, Vec<u8>));

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            self.pos += 1;
            return Some((self.rh.current_hash(), (0, self.sub.clone())));
        }
        if let Some(&next) = self.seq.next() {
            let prev = self.sub[0];
            self.sub.remove(0);
            self.sub.push(next);
            self.rh.slide(prev, next);
            self.pos += 1;
            return Some((self.rh.current_hash(), (self.pos - 1, self.sub.clone())));
        }
        None
    }
}

pub struct IntervalSubSequenceHashIterator<'a> {
    seq: &'a mut Iter<'a, u8>,
    pos: usize,
    length: usize,
    m: usize,
}

impl<'a> IntervalSubSequenceHashIterator<'a> {
    pub fn new(seq: &'a mut Iter<'a, u8>, length: usize, m: usize) -> Self {
        Self {
            seq,
            pos: 0,
            length,
            m,
        }
    }
}

impl Iterator for IntervalSubSequenceHashIterator<'_> {
    type Item = (u64, (usize, Vec<u8>));

    fn next(&mut self) -> Option<Self::Item> {
        let mut sub = vec![];
        for _ in 0..self.length {
            if let Some(&next) = self.seq.next() {
                sub.push(next)
            } else {
                return None;
            }
        }
        let rh = RollingHash::new(&sub[..]);
        let hash = rh.current_hash();
        for _ in 0..(self.m.saturating_sub(self.length)) {
            if self.seq.next().is_none() {
                return None;
            }
        }
        self.pos += self.m;
        Some((hash, (self.pos - self.m, sub.clone())))
    }
}

pub struct ExactSubMatchesIterator<'a> {
    sub_seq_hash_iterator: SubSequenceHashIterator<'a>,
    multi_dict: Multidict<(usize, Vec<u8>)>,
}

impl<'a> ExactSubMatchesIterator<'a> {
    pub fn new(a: &'a mut Iter<'a, u8>, b: &'a mut Iter<'a, u8>, length: usize, m: usize) -> Self {
        let b_iterator = SubSequenceHashIterator::new(b, length);
        let a_iterator = IntervalSubSequenceHashIterator::new(a, length, m);
        // let a_iterator = SubSequenceHashIterator::new(a, length);
        let multi_dict: Multidict<(usize, Vec<u8>)> = Multidict::new(a_iterator.collect());
        Self {
            sub_seq_hash_iterator: b_iterator,
            multi_dict,
        }
    }
}

impl Iterator for ExactSubMatchesIterator<'_> {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((h_b, (b_pos, b_sub))) = self.sub_seq_hash_iterator.next() {
            let mut result = vec![];
            for (a_pos, a_sub) in self.multi_dict.get(&h_b) {
                if a_sub != b_sub {
                    continue;
                }
                result.push((a_pos, b_pos));
            }
            if !result.is_empty() {
                return Some(result);
            }
        }
        None
    }
}

mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
