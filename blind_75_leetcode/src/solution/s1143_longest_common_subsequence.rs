#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let l1 = text1.len();
        let l2 = text2.len();
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 1..=l1 {
            for j in 1..=l2 {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[l1][l2]
    }
}

#[cfg(test)]
mod tests_1143 {
    use super::*;

    fn helper(text1: &str, text2: &str) -> i32 {
        Solution::longest_common_subsequence(text1.to_string(), text2.to_string())
    }

    #[test]
    fn it_works() {
        assert_eq!(helper("aa", "aaa"), 2);
        assert_eq!(helper("pqr", "qrpz"), 2);
        assert_eq!(helper("acfhgc", "afgch"), 4);
        assert_eq!(helper("abcefhgch", "aefgc"), 5);
        assert_eq!(helper("abcde", "ace"), 3);
        assert_eq!(helper("abc", "abc"), 3);
        assert_eq!(helper("abc", "def"), 0);
        assert_eq!(helper("abccd", "bccd"), 4);
        assert_eq!(helper("abcfcd", "bcd"), 3);
        assert_eq!(helper("acbfd", "bd"), 2);
        assert_eq!(helper("ezupkr", "ubmrapg"), 2);
        assert_eq!(helper("oxcpqrsvwf", "shmtulqrypy"), 2);
        assert_eq!(helper("pmjghexybyrgzczy", "hafcdqbgncrcbihkd"), 4);
        assert_eq!(helper("bsbininm", "jmjkbkjkv"), 1);
        assert_eq!(helper("aaaa", "aaaaa"), 4);
        assert_eq!(
            helper("yzebsbuxmtcfmtodclszgh", "ejevmhcvshclydqrulwbyha"),
            6
        );
        assert_eq!(helper("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 210);
    }
}
