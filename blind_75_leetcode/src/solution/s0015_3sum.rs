#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut nums = nums;
        nums.sort();
        let l = nums.len();
        let mut map = HashMap::new();
        for i in 0..l {
            map.insert(nums[i], i);
        }

        let mut ans = HashSet::new();

        for i in 0..l {
            let target = nums[i];
            for j in (i + 1)..l {
                if let Some(&index) = map.get(&(-target - nums[j])) {
                    if index <= j {
                        continue;
                    }
                    let res = vec![target, nums[j], -target - nums[j]];
                    if !ans.insert(res.clone()) {
                        continue;
                    }
                }
            }
        }
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod tests_15 {
    use super::*;
    fn assert_equal(left: &[Vec<i32>], right: &[Vec<i32>]) {
        use std::collections::HashSet;
        let left: HashSet<&Vec<i32>> = left.iter().collect();
        let right: HashSet<&Vec<i32>> = right.iter().collect();
        if left != right {
            assert_eq!(left, right)
        }
    }

    #[test]
    fn it_works() {
        assert_equal(
            &Solution::three_sum(vec![-1, 2, -1]),
            &vec![vec![-1, -1, 2]],
        );
        assert_equal(
            &Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            &vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        );
        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());
        assert_equal(&Solution::three_sum(vec![0, 0, 0]), &vec![vec![0, 0, 0]]);
    }
}
