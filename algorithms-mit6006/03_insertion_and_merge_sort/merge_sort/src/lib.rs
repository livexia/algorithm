use std::fmt::Debug;

fn merge_sort<T>(nums: &Vec<T>) -> Vec<T>
where
    T: Clone + Copy + Ord + Debug,
{
    let length = nums.len();
    if length < 2 {
        return nums.clone();
    }
    let mid = length / 2;
    let l = merge_sort(&nums[..mid].to_vec());
    let r = merge_sort(&nums[mid..].to_vec());
    merge(&l, &r)
}

fn merge<T>(l: &Vec<T>, r: &Vec<T>) -> Vec<T>
where
    T: Clone + Copy + Ord + Debug,
{
    let mut result = vec![];
    let n = l.len();
    let m = r.len();
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if l[i] > r[j] {
            result.push(r[j]);
            j += 1;
        } else {
            result.push(l[i]);
            i += 1;
        }
    }
    result.extend(r[j..].iter());
    result.extend(l[i..].iter());
    result
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        let mut nums = vec![5, 4, 3, 2, 1, -5, 10];
        nums = merge_sort(&mut nums);
        assert_eq!(nums, vec![-5, 1, 2, 3, 4, 5, 10]);
    }

    #[test]
    fn rand_test() {
        let mut nums = [0i32; 100000];
        thread_rng().fill(&mut nums[..]);
        let mut nums = nums.to_vec();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        nums = merge_sort(&nums);
        assert_eq!(nums, sorted_nums);
    }
}
