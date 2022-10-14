#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s: Vec<u32> = s
            .bytes()
            .into_iter()
            .map(|b| 1 << (b - 'a' as u8))
            .collect();
        let mut shown = vec![s[0]];
        let mut counts = vec![0];
        for c in s {
            // 在当前出现表中不存在
            let mut flag = false;
            for i in (0..shown.len()).rev() {
                if c & shown[i] != 0 {
                    // 在之前的集中出现
                    shown[i] = shown[i..].iter().fold(0, |s, t| s | t);
                    shown.truncate(i + 1);
                    counts[i] = counts[i..].iter().sum::<i32>() + 1;
                    counts.truncate(i + 1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                shown.push(c);
                counts.push(1);
            }
        }
        counts
    }
}

#[cfg(test)]
mod tests_763 {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }
}
