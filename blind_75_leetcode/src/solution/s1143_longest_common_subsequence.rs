#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = if text1.len() < text2.len() {
            (text1, text2)
        } else {
            (text2, text1)
        };

        let mut shown = vec![0; 26];

        let mut pos = vec![];
        for (index1, char1) in text1.char_indices() {
            let offset = shown[(char1 as u8 - 'a' as u8) as usize];
            println!("- {:?}, {}, {}, {}", pos, index1, char1, offset);
            for (index2, char2) in text2.char_indices().skip(offset) {
                if char1 == char2 {
                    pos.push(index2);
                    shown[(char2 as u8 - 'a' as u8) as usize] = index2 + 1;
                    break;
                }
            }
        }
        println!("- {:?}", pos);
        if pos.is_empty() {
            return 0;
        }

        let mut dp = vec![];
        dp.push(&pos[0]);
        for index in 1..pos.len() {
            if &pos[index] > dp.last().unwrap() {
                dp.push(&pos[index]);
            } else {
                match dp.binary_search(&&pos[index]) {
                    Ok(_) => (),
                    Err(j) => {
                        dp[j] = &pos[index];
                    }
                }
            }
        }
        println!("{:?}", dp);
        dp.len() as i32
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
