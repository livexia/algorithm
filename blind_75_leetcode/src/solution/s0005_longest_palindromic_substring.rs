#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let n = s.len();
        let mut result = (0, 0);
        for i in 0..n {
            let p1 = Solution::inside_out(i, i, &s);
            let p2 = Solution::inside_out(i, i + 1, &s);
            let p = if p1.1 - p1.0 > p2.1 - p2.0 { p1 } else { p2 };
            if result.1 - result.0 < p.1 - p.0 {
                result = p
            }
        }
        s[result.0..result.1].iter().collect()
    }

    fn inside_out(left: usize, right: usize, s: &[char]) -> (usize, usize) {
        let (mut left, mut right) = (left as i32, right);
        while left >= 0 && right < s.len() && s[left as usize] == s[right] {
            left -= 1;
            right += 1;
        }
        ((left + 1) as usize, right)
    }

    fn dp(i: usize, j: usize, s: &[char]) -> bool {
        i == j || s[i] == s[j] && (i + 1 == j || Solution::dp(i + 1, j - 1, s))
    }
}

#[cfg(test)]
mod tests_5 {
    use super::*;

    #[test]
    fn it_works() {
        assert!(["bab".to_string(), "aba".to_string()]
            .contains(&Solution::longest_palindrome("babad".to_string())));
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("reifadyqgztixemwswtccodfnchcovrmiooffbbijkecuvlvukecutasfxqcqygltrogrdxlrslbnzktlanycgtniprjlospzhhgdrqcwlukbpsrumxguskubokxcmswjnssbkutdhppsdckuckcbwbxpmcmdicfjxaanoxndlfpqwneytatcbyjmimyawevmgirunvmdvxwdjbiqszwhfhjmrpexfwrbzkipxfowcbqjckaotmmgkrbjvhihgwuszdrdiijkgjoljjdubcbowvxslctleblfmdzmvdkqdxtiylabrwaccikkpnpsgcotxoggdydqnuogmxttcycjorzrtwtcchxrbbknfmxnonbhgbjjypqhbftceduxgrnaswtbytrhuiqnxkivevhprcvhggugrmmxolvfzwadlnzdwbtqbaveoongezoymdrhywxcxvggsewsxckucmncbrljskgsgtehortuvbtrsfisyewchxlmxqccoplhlzwutoqoctgfnrzhqctxaqacmirrqdwsbdpqttmyrmxxawgtjzqjgffqwlxqxwxrkgtzqkgdulbxmfcvxcwoswystiyittdjaqvaijwscqobqlhskhvoktksvmguzfankdigqlegrxxqpoitdtykfltohnzrcgmlnhddcfmawiriiiblwrttveedkxzzagdzpwvriuctvtrvdpqzcdnrkgcnpwjlraaaaskgguxzljktqvzzmruqqslutiipladbcxdwxhmvevsjrdkhdpxcyjkidkoznuagshnvccnkyeflpyjzlcbmhbytxnfzcrnmkyknbmtzwtaceajmnuyjblmdlbjdjxctvqcoqkbaszvrqvjgzdqpvmucerumskjrwhywjkwgligkectzboqbanrsvynxscpxqxtqhthdytfvhzjdcxgckvgfbldsfzxqdozxicrwqyprgnadfxsionkzzegmeynye".to_string()),
            "uvlvu".to_string()
        );
    }
}
