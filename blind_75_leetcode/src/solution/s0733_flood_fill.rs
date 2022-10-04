#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let old_color = image[sr as usize][sc as usize];
        // with dfs
        // Solution::dfs(&mut image, sr as usize, sc as usize, old_color, color);

        //with bfs
        Solution::bfs(&mut image, sr as usize, sc as usize, old_color, color);
        image
    }

    fn bfs(image: &mut [Vec<i32>], sr: usize, sc: usize, old_color: i32, color: i32) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((sr, sc));
        while let Some((r, c)) = queue.pop_front() {
            if image[r][c] == color {
                continue;
            }
            image[r][c] = color;
            if r > 0 && image[r - 1][c] == old_color {
                queue.push_back((r - 1, c))
            }
            if r < image.len() - 1 && image[r + 1][c] == old_color {
                queue.push_back((r + 1, c))
            }
            if c > 0 && image[r][c - 1] == old_color {
                queue.push_back((r, c - 1))
            }
            if c < image[0].len() - 1 && image[r][c + 1] == old_color {
                queue.push_back((r, c + 1))
            }
        }
    }

    fn dfs(image: &mut [Vec<i32>], sr: usize, sc: usize, old_color: i32, color: i32) {
        if image[sr][sc] == color {
            return;
        }
        image[sr][sc] = color;
        if sr > 0 && image[sr - 1][sc] == old_color {
            Solution::dfs(image, sr - 1, sc, old_color, color);
        }
        if sr < image.len() - 1 && image[sr + 1][sc] == old_color {
            Solution::dfs(image, sr + 1, sc, old_color, color);
        }
        if sc > 0 && image[sr][sc - 1] == old_color {
            Solution::dfs(image, sr, sc - 1, old_color, color);
        }
        if sc < image[0].len() - 1 && image[sr][sc + 1] == old_color {
            Solution::dfs(image, sr, sc + 1, old_color, color);
        }
    }
}

#[cfg(test)]
mod tests_733 {
    use super::*;
    use crate::leetcode_vec;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::flood_fill(leetcode_vec![[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2),
            leetcode_vec![[2, 2, 2], [2, 2, 0], [2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(leetcode_vec![[0, 0, 0], [0, 0, 0]], 0, 0, 2),
            leetcode_vec![[2, 2, 2], [2, 2, 2]]
        );
    }
}
