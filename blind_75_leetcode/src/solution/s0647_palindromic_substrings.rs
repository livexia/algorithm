#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<_> = s.chars().collect();
        (0..s.len()).fold(0, |c, i| {
            c + Solution::count_palindromic(i, i, &s) + Solution::count_palindromic(i, i + 1, &s)
        }) as i32
    }

    fn count_palindromic(mut i: usize, mut j: usize, s: &[char]) -> usize {
        let mut count = 0;
        while j < s.len() && s[i] == s[j] {
            count += 1;
            if i == 0 {
                break;
            }
            i -= 1;
            j += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests_647 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }
}
