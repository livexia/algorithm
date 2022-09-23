#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut nums = nums;
        nums.sort();
        let l = nums.len();
        let mut ans = vec![];

        let mut shown = HashSet::new();

        for i in 0..l {
            let mut map = HashMap::new();
            let target = nums[i];
            for j in 0..l {
                if j == i {
                    continue;
                }
                match map.get(&(-target - nums[j])) {
                    Some(_) => {
                        let mut res = vec![target, -target - nums[j], nums[j]];
                        res.sort();
                        if shown.insert(res.clone()) {
                            ans.push(res);
                        }
                    }
                    None => {
                        map.insert(nums[j], j);
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests_15 {
    use super::*;
    fn is_equal(left: &[Vec<i32>], right: &[Vec<i32>]) -> bool {
        use std::collections::HashSet;
        let left: HashSet<&Vec<i32>> = left.iter().collect();
        let right: HashSet<&Vec<i32>> = right.iter().collect();
        left == right
    }

    #[test]
    fn it_works() {
        assert!(is_equal(
            &Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            &vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        ));
        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());
        assert!(is_equal(
            &Solution::three_sum(vec![0, 0, 0]),
            &vec![vec![0, 0, 0]]
        ));
    }
}
