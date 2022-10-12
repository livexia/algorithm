#![allow(dead_code)]
struct RangeModule {
    list: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut res = vec![];
        let (mut left, mut right) = (left, right);
        let mut placed = false;
        for &(start, end) in &self.list {
            if start > right {
                if !placed {
                    placed = true;
                    res.push((left, right));
                }
                res.push((start, end));
            } else if left > end {
                res.push((start, end));
            } else {
                left = left.min(start);
                right = right.max(end);
            }
        }
        res.push((left, right));
        self.list = res;
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        for &(start, end) in &self.list {
            if left >= start && right <= end {
                return true;
            }
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut res = vec![];
        let (mut left, right) = (left, right);
        for &(start, end) in &self.list {
            if start > right {
                res.push((start, end));
            } else if left > end {
                res.push((left, right));
            } else {
                let (mut nl, mut nr) = (start, end);
                if left <= start && right >= end {
                    left = end;
                    continue;
                } else if left <= start && right < end {
                    left = right;
                    nl = right;
                } else if left > start && right <= end {
                    res.push((start, left));
                    nl = right;
                } else if left > start && right > end {
                    res.push((start, left));
                } else {
                    unreachable!()
                }
                if nl < nr {
                    res.push((nl, nr))
                }
            }
        }
        self.list = res;
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */

#[cfg(test)]
mod tests_715 {
    use super::*;

    #[test]
    fn it_works() {
        let mut range = RangeModule::new();
        range.add_range(10, 20);
        assert_eq!(range.list, vec![(10, 20)]);
        range.remove_range(14, 16);
        assert_eq!(range.list, vec![(10, 14), (16, 20)]);
        assert_eq!(range.query_range(10, 14), true);
        assert_eq!(range.query_range(13, 15), false);
        assert_eq!(range.query_range(16, 17), true);
    }
}
