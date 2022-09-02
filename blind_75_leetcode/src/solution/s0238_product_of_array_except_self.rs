#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_count = 0;
        let mut zero_index = 0;
        let total_product = nums.iter().enumerate().fold(1, |s, (index, num)| {
            if num == &0 {
                zero_count += 1;
                zero_index = index;
                s
            } else {
                s * num
            }
        });
        let mut answer = vec![];
        if zero_count > 1 {
            answer = vec![0; nums.len()];
        } else if zero_count == 1 {
            answer = vec![0; nums.len()];
            answer[zero_index] = total_product;
        } else {
            for num in nums {
                answer.push(total_product / num)
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests_238 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
