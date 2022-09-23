#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut nums = nums;
        nums.sort();
        let l = nums.len();
        let mut ans = vec![];

        for i in 0..l {
            let mut map = HashMap::new();
            let target = nums[i];
            for j in 0..l {
                if j == i {
                    continue;
                }
                match map.get(&(-target - nums[j])) {
                    Some(_) => ans.push(vec![target, -target - nums[j], nums[j]]),
                    None => {
                        map.insert(nums[j], j);
                    }
                };
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_15 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
