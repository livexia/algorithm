#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut tail = vec![nums[0]];

        for i in 1..nums.len() {
            if &nums[i] > tail.last().unwrap() {
                tail.push(nums[i])
            } else {
                match Solution::binary_search(&tail, &nums[i]) {
                    // match tail.binary_search(&nums[i]) {
                    Ok(_) => (),
                    Err(j) => tail[j] = nums[i],
                }
            }
        }
        tail.len() as i32
    }

    fn binary_search(v: &[i32], x: &i32) -> Result<usize, usize> {
        let mut left = 0;
        let mut right = v.len() - 1;
        while left < right {
            let mid = (left + right) >> 1;
            if x == &v[mid] {
                return Ok(mid);
            } else if x < &v[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        Err(left)
    }
}

#[cfg(test)]
mod tests_300 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
