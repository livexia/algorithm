/**
一维数组的peak查找，复杂度O(log2n)

利用rand生成随机数组，根据得到的结果，判断结果是否真为peak
*/

fn one_dimensional_peak_finding<T>(nums: &[T], start: usize) -> Option<(T, usize)> 
where
    T: std::cmp::PartialOrd + Clone + Copy
{
    if nums.is_empty() {
        return None
    }
    let mid = nums.len() / 2;
    if mid > 0 && nums[mid] < nums[mid-1] {
        one_dimensional_peak_finding(&nums[..mid], start)
    } else if mid + 1 < nums.len() && nums[mid] < nums[mid+1] {
        one_dimensional_peak_finding(&nums[mid+1..], start+mid+1)
    } else {
        Some((nums[mid], mid + start))
    }
}

fn is_peak<T>(nums: &[T], index: usize) -> bool 
where
    T: std::cmp::PartialOrd + Clone + Copy
{
    let mut answer = false;
    if index > 0 {
        if nums[index] >= nums[index-1] {
            answer = true;
        } else {
            answer = false;
        }
    }
    if index + 1 < nums.len() {
        if nums[index] >= nums[index+1] {
            answer = true;
        } else {
            answer = false;
        }
    }
    if index == 0 && nums.len() == 1 {
        answer = true;
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::{one_dimensional_peak_finding, is_peak};
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        for _ in 0..10_000 {
            let mut nums = [0i8; 256];
            thread_rng().fill(&mut nums);
            if let Some((_, index)) = one_dimensional_peak_finding(&nums, 0) {
                assert!(is_peak(&nums, index));
            } else {
                println!("{:?}", nums);
                assert!(false);
            }
        }
    }
}
