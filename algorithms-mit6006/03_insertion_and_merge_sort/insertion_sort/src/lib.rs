use std::fmt::Debug;

fn insertion_sort<T>(nums: &mut Vec<T>)
where
    T: Clone + Copy + PartialEq + Eq + Ord + Debug,
{
    let n = nums.len();
    for i in 1..n {
        swap(nums, i);
    }
}

fn swap<T>(nums: &mut Vec<T>, cur: usize) 
where
    T: Clone + Copy + PartialEq + Eq + Ord,
{
    for i in (1..=cur).rev() {
        if nums[i] < nums[i - 1] {
            let temp = nums[i];
            nums[i] = nums[i - 1];
            nums[i - 1] = temp;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort;
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        let mut nums = vec![5, 4, 3, 2, 1, -5, 10];
        insertion_sort(&mut nums);
        assert_eq!(nums, vec![-5, 1, 2, 3, 4, 5, 10]);
    }

    #[test]
    fn rand_test() {
        let mut nums = [0i32; 10000];
        thread_rng().fill(&mut nums[..]);
        let mut nums = nums.to_vec();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        insertion_sort(&mut nums);
        assert_eq!(nums, sorted_nums);
    }
}
