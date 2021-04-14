/**
radix sort: k = n**O(1) => k <= n\**100
    1. imagine each integer as base b
    2. #digits = d = logb(k) + 1
    3. min when b == n
    4. sort (all n items) by least significant digit → can extract in O(1) time
    5.  · · ·
    6. sort by most significant digit → can extract in O(1) time
        - sort must be stable: preserve relative order of items with the same key
        - =⇒ don’t mess up previous sorting

need to be integer so chose usize for the radix sort
*/

fn radix_sort(nums: &Vec<usize>) -> Vec<usize> {
    let base = nums.len();
    let k = nums
        .iter()
        .map(|&x| (x as f64).log(base as f64).ceil() as usize)
        .max()
        .unwrap();
    let mut prev_nums = nums.clone();
    let mut count: Vec<Vec<usize>> = vec![vec![]; base];
    for i in 0..k {
        for &x in &prev_nums {
            let digit = x / base.pow(i as u32) % base;
            count[digit].push(x);
        }
        prev_nums.clear();
        for v in &count {
            prev_nums.extend(v.iter())
        }
        count = vec![vec![]; base];
    }
    prev_nums
}


#[cfg(test)]
mod tests {
    use crate::radix_sort;
    use rand::{Rng, thread_rng};

    #[test]
    fn it_works() {
        let nums: Vec<usize> = vec![100, 1002, 1004, 12345, 1, 2, 3, 5, 6, 10];

        let mut sorted = nums.clone();
        sorted.sort();
        assert_eq!(radix_sort(&nums), sorted);
    }

    #[test]
    fn rand_test() {
        let mut nums = [0usize; 258489];
        thread_rng().fill(&mut nums[..]);
        let nums = nums.to_vec();

        let mut sorted = nums.clone();
        sorted.sort();
        assert_eq!(radix_sort(&nums), sorted);
    }

    #[test]
    fn test2() {
        let nums: Vec<usize> = (0..232347).rev().collect();

        let mut sorted = nums.clone();
        sorted.sort();
        assert_eq!(radix_sort(&nums), sorted);
    }
}
