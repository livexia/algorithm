#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut graph: Vec<usize> = vec![num_courses + 1; num_courses];
        for prerequisite in prerequisites {
            graph[prerequisite[1] as usize] = prerequisite[0] as usize;
        }
        println!("{:?}", graph);
        for course in 0..num_courses as usize {
            let mut next_course = course;
            while graph[next_course] != num_courses + 1 {
                next_course = graph[next_course];
                if next_course == course {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests_207 {
    use super::*;

    fn can_finish(num_courses: i32, prerequisites: Vec<[i32; 2]>) -> bool {
        Solution::can_finish(
            num_courses,
            prerequisites.into_iter().map(|p| p.to_vec()).collect(),
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(can_finish(3, vec![[0, 2], [1, 2], [2, 0]]), false);
        assert_eq!(
            can_finish(
                20,
                vec![
                    [0, 10],
                    [3, 18],
                    [5, 5],
                    [6, 11],
                    [11, 14],
                    [13, 1],
                    [15, 1],
                    [17, 4]
                ]
            ),
            false
        );
    }
}
