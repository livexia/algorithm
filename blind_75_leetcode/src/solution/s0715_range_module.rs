#![allow(dead_code)]

struct RangeModule {
    range: Option<(i32, i32)>,
    next: Option<Box<RangeModule>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self {
            range: None,
            next: None,
        }
    }

    fn with_range(left: i32, right: i32) -> Self {
        Self {
            range: Some((left, right)),
            next: None,
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);
        if let Some((start, end)) = self.range {
            if start > right {
                let mut temp_r = RangeModule::with_range(left, right);
                use std::mem::swap;
                swap(&mut temp_r, self);
                self.next = Some(Box::new(temp_r));
            } else if left > end {
                if self.next.is_none() {
                    self.next = Some(Box::new(RangeModule::with_range(left, right)))
                } else {
                    self.next.as_mut().unwrap().add_range(left, right)
                }
            } else {
                left = left.min(start);
                right = right.max(end);
                if self.next.is_none() {
                    self.range = Some((left, right))
                } else {
                    let next = self.next.take().unwrap();
                    *self = *next;
                    self.add_range(left, right)
                }
            }
        } else {
            self.range = Some((left, right))
        }
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if let Some((start, end)) = self.range {
            if left >= start && right <= end {
                return true;
            }
            if let Some(next) = &self.next {
                return next.query_range(left, right);
            }
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if let Some((start, end)) = self.range {
            if start <= right && left <= end {
                let mut removed = false;
                if left > start {
                    removed = true;
                    self.range = Some((start, left));
                }
                if right < end {
                    if !removed {
                        self.range = Some((right, end));
                    } else {
                        let mut temp_r = RangeModule::with_range(right, end);
                        temp_r.next = self.next.take();
                        self.next = Some(Box::new(temp_r));
                    }
                    removed = true;
                }
                if !removed {
                    if self.next.is_none() {
                        self.range = None
                    } else {
                        *self = *self.next.take().unwrap();
                        self.remove_range(left, right);
                    }
                }
            }
            if let Some(next) = &mut self.next {
                next.remove_range(left, right)
            }
        }
    }

    fn list(&self) -> Vec<(i32, i32)> {
        let mut res = vec![];
        if let Some(r) = self.range {
            res.push(r);
            if let Some(next) = &self.next {
                res.extend(next.list())
            }
        }
        res
    }
}

struct RangeModuleWithVec {
    list: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModuleWithVec {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        if self.list.is_empty() {
            self.list.push((left, right));
            return;
        }
        let start = match self.list.binary_search_by_key(&left, |&(a, _)| a) {
            Err(n) => {
                if n == 0 || self.list[n - 1].1 < left {
                    n
                } else {
                    n.saturating_sub(1)
                }
            }
            Ok(n) => n,
        };
        let end = match self.list.binary_search_by_key(&right, |&(a, _)| a) {
            Err(n) => n,
            Ok(n) => n + 1,
        };
        if start == end {
            self.list.insert(start, (left, right))
        }
        let left = left.min(self.list[start].0);
        let right = right.max(self.list[end - 1].1);
        self.list[start] = (left, right);
        for _ in start + 1..end {
            if start + 1 < self.list.len() {
                self.list.remove(start + 1);
            }
        }
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let start = match self.list.binary_search(&(left, right)) {
            Err(n) => {
                if n > 0 && self.list[n - 1].1 > left {
                    n - 1
                } else {
                    if n == self.list.len() {
                        return false;
                    }
                    n
                }
            }
            Ok(_) => return true,
        };
        self.list[start].0 <= left && self.list[start].1 >= right
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut res = vec![];
        for &(start, end) in &self.list {
            if start > right || left > end {
                res.push((start, end));
            } else {
                if left > start {
                    res.push((start, left));
                }
                if right < end {
                    res.push((right, end));
                }
            }
        }
        self.list = res;
    }

    fn list(&self) -> Vec<(i32, i32)> {
        self.list.clone()
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
    use super::RangeModuleWithVec as RangeModule;
    use crate::leetcode_vec;

    fn helper(calls: Vec<&str>, args: Vec<Vec<i32>>) -> Vec<Option<bool>> {
        let mut res = vec![None; calls.len()];
        if calls[0] != "RangeModule" {
            unreachable!("Wrong Input not start with 'RangeModule'");
        }
        if calls.len() != args.len() {
            unreachable!(
                "Wrong Input: the number of calls not equal to the number of args, {} != {}",
                calls.len(),
                args.len()
            );
        }
        let mut r = RangeModule::new();
        for (i, (call, arg)) in calls[1..].iter().zip(args[1..].iter()).enumerate() {
            match call {
                &"queryRange" => res[i + 1] = Some(r.query_range(arg[0], arg[1])),
                &"addRange" => r.add_range(arg[0], arg[1]),
                &"removeRange" => r.remove_range(arg[0], arg[1]),
                _ => unreachable!("Wrong Input: {}", call),
            };
        }
        res
    }

    #[test]
    fn it_works() {
        let mut range = RangeModule::new();
        range.add_range(10, 20);
        assert_eq!(range.list(), vec![(10, 20)]);
        range.remove_range(14, 16);
        assert_eq!(range.list(), vec![(10, 14), (16, 20)]);
        assert_eq!(range.query_range(10, 14), true);
        assert_eq!(range.query_range(13, 15), false);
        assert_eq!(range.query_range(16, 17), true);
    }

    #[test]
    fn it_works1() {
        let mut range = RangeModule::new();
        range.add_range(1, 2);
        range.add_range(9, 76);
        range.remove_range(2, 77);
        assert_eq!(range.query_range(31, 66), false);
    }

    #[test]
    fn it_works2() {
        let calls = vec![
            "RangeModule",
            "addRange",
            "addRange",
            "addRange",
            "queryRange",
        ];
        let args = leetcode_vec![[], [37, 55], [30, 97], [1, 36], [18, 96]];
        let expected = vec![None, None, None, None, Some(true)];
        let res = helper(calls, args);
        assert_eq!(expected.len(), res.len());
        assert_eq!(expected, res);
    }
}
