#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() & 1 == 1 {
            return false;
        }
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                ')' | ']' | '}' => {
                    if let Some(c1) = stack.pop() {
                        if !((c == ')' && c1 == '(')
                            || (c == ']' && c1 == '[')
                            || (c == '}' && c1 == '{'))
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests_20 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
}
