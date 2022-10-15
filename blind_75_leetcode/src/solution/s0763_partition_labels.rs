#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // with bit manipulation
        // Solution::partition_labels_with_bit(s)

        // with interval
        // Solution::partition_labels_with_interval(s)

        // with char last pos
        Solution::partition_labels_with_last_pos(s)
    }

    pub fn partition_labels_with_last_pos(s: String) -> Vec<i32> {
        let mut last = vec![0; 26];
        let s: Vec<(usize, usize)> = s
            .bytes()
            .map(|c| (c - 'a' as u8) as usize)
            .enumerate()
            .collect();
        for &(i, c) in &s {
            last[c] = i;
        }
        let (mut start, mut end) = (0, 0);
        let mut res = vec![];
        for (i, c) in s {
            end = end.max(last[c]);
            if i == end {
                res.push((end - start) as i32 + 1);
                start = end + 1;
            }
        }
        res
    }

    pub fn partition_labels_with_interval(s: String) -> Vec<i32> {
        let mut pos = vec![26; 26];
        let s: Vec<u8> = s.bytes().map(|b| b - 'a' as u8).collect();
        let mut intervals = vec![];
        for (i, c) in s.into_iter().enumerate() {
            if pos[c as usize] == 26 {
                intervals.push((i, i));
                pos[c as usize] = intervals.len() - 1;
            } else {
                intervals[pos[c as usize]].1 = i;
            }
        }
        let mut res = vec![-1];
        let mut last = intervals[0].1;
        for &(start, end) in &intervals[1..] {
            if last < start {
                res.push(last as i32);
            }
            last = last.max(end);
        }
        res.push(last as i32);
        res.windows(2).map(|w| w[1] - w[0]).collect()
    }

    pub fn partition_labels_with_bit(s: String) -> Vec<i32> {
        let s: Vec<u32> = s
            .bytes()
            .into_iter()
            .map(|b| 1 << (b - 'a' as u8))
            .collect();
        let mut shown = vec![s[0]];
        let mut counts = vec![0];
        for c in s {
            let mut flag = true;
            for i in (0..shown.len()).rev() {
                if c & shown[i] != 0 {
                    // 在之前的集中出现
                    shown[i] = shown[i..].iter().fold(0, |s, t| s | t);
                    shown.truncate(i + 1);
                    counts[i] = counts[i..].iter().sum::<i32>() + 1;
                    counts.truncate(i + 1);
                    flag = false;
                    break;
                }
            }
            if flag {
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
