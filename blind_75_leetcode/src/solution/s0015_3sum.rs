#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let l = nums.len();
        let mut ans = vec![];

        for i in 0..l {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = nums[i];
            let mut third = l - 1;
            for j in (i + 1)..l {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                while j < third && nums[j] + nums[third] > -target {
                    third -= 1;
                }
                if third == j {
                    break;
                }
                if nums[j] + nums[third] == -target {
                    ans.push(vec![target, nums[j], nums[third]])
                }
            }
        }
        ans
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
