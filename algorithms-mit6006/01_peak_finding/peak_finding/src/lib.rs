pub mod peak {
    use std::error::Error;
    use std::result;

    macro_rules! err {
        ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
    }

    type Result<T> = result::Result<T, Box<dyn Error>>;

    #[derive(Debug, Clone)]
    pub struct PeakProblem {
        array: Vec<Vec<i32>>,
        bounds: (i32, i32, i32, i32),
        start_row: i32,
        start_col: i32,
        num_row: i32,
        num_col: i32,
    }

    impl PeakProblem {
        pub fn new(array: Vec<Vec<i32>>, bounds: (i32, i32, i32, i32)) -> Self {
            let (start_row, start_col, num_row, num_col) = bounds;
            Self {
                array,
                bounds,
                start_row,
                start_col,
                num_row,
                num_col,
            }
        }

        pub fn get(&self, location: (i32, i32)) -> Option<i32> {
            let (r, c) = location;
            if r < 0 || r >= self.num_row {
                return None;
            }
            if c < 0 || c >= self.num_col {
                return None;
            }
            Some(self.array[(self.start_row + r) as usize][(self.start_col + c) as usize])
        }

        pub fn get_better_neighbor(&self, location: (i32, i32)) -> (i32, i32) {
            let (r, c) = location;
            let mut best = location;
            if r - 1 >= 0 && self.get((r - 1, c)) > self.get(best) {
                best = (r - 1, c)
            }
            if c - 1 >= 0 && self.get((r, c - 1)) > self.get(best) {
                best = (r, c - 1)
            }
            if r + 1 < self.num_row && self.get((r + 1, c)) > self.get(best) {
                best = (r + 1, c)
            }
            if c + 1 < self.num_col && self.get((r, c + 1)) > self.get(best) {
                best = (r, c + 1)
            }

            best
        }

        pub fn get_max(&self, locations: Vec<(i32, i32)>) -> (i32, i32) {
            let (mut best_loc, mut best_val) = (locations[0], 0);
            for loc in locations {
                if let Some(val) = self.get(loc) {
                    if val > best_val {
                        best_loc = loc;
                        best_val = val;
                    }
                }
            }
            best_loc
        }

        pub fn is_peak(&self, location: (i32, i32)) -> bool {
            self.get_better_neighbor(location) == location
        }

        pub fn get_sub_problem(&self, bounds: (i32, i32, i32, i32)) -> Self {
            let (s_row, s_col, num_row, num_col) = bounds;
            let new_bounds = (
                self.start_row + s_row,
                self.start_col + s_col,
                num_row,
                num_col,
            );
            Self::new(self.array.clone(), new_bounds)
        }

        pub fn get_sub_problem_containing(
            &self,
            bound_list: Vec<(i32, i32, i32, i32)>,
            location: (i32, i32),
        ) -> Self {
            let (row, col) = location;
            for (s_row, s_col, n_row, n_col) in bound_list {
                if s_row <= row && row < s_row + n_row {
                    if s_col <= col && col < s_col + n_col {
                        return self.get_sub_problem((s_row, s_col, n_row, n_col));
                    }
                }
            }
            self.clone()
        }

        pub fn get_location_in_self(&self, problem: &Self, location: (i32, i32)) -> (i32, i32) {
            let (r, c) = location;
            let new_row = r + problem.start_row - self.start_row;
            let new_col = c + problem.start_col - self.start_col;
            (new_row, new_col)
        }

        pub fn create_problem(array: Vec<Vec<i32>>) -> Self {
            let (r, c) = get_dimensions(&array);
            return Self::new(array, (0, 0, r, c));
        }
    }

    pub fn get_dimensions(array: &Vec<Vec<i32>>) -> (i32, i32) {
        let rows = array.len();
        let mut cols = 0;
        for row in array {
            cols = cols.max(row.len())
        }
        (rows as i32, cols as i32)
    }

    pub fn cross_product(list1: &Vec<i32>, list2: &Vec<i32>) -> Vec<(i32, i32)> {
        let mut answer = vec![];
        for &a in list1 {
            for &b in list2 {
                answer.push((a, b))
            }
        }
        answer
    }

    pub fn algorithm1(problem: &PeakProblem) -> Result<(i32, i32)> {
        if problem.num_row <= 0 || problem.num_col <= 0 {
            return err!("problem {:?} is empty", problem);
        }
        let mid = problem.num_col / 2;
        let (sub_start_r, sub_num_r) = (0, problem.num_row);
        let (sub_start_c1, sub_num_c1) = (0, mid);
        let (sub_start_c2, sub_num_c2) = (mid + 1, problem.num_col - (mid + 1));
        let mut sub_problems = vec![];
        sub_problems.push((sub_start_r, sub_start_c1, sub_num_r, sub_num_c1));
        sub_problems.push((sub_start_r, sub_start_c2, sub_num_r, sub_num_c2));

        let divider = cross_product(&(0..problem.num_row).collect(), &vec![mid]);
        let best_loc = problem.get_max(divider);
        let neighbor = problem.get_better_neighbor(best_loc);

        if neighbor == best_loc {
            return Ok(best_loc);
        }

        let sub = problem.get_sub_problem_containing(sub_problems, neighbor);
        let result = algorithm1(&sub)?;
        Ok(problem.get_location_in_self(&sub, result))
    }

    pub fn algorithm2(problem: &PeakProblem, location: (i32, i32)) -> Result<(i32, i32)> {
        if problem.num_row <= 0 || problem.num_col <= 0 {
            return err!("problem {:?} is empty", problem);
        }
        let next_location = problem.get_better_neighbor(location);
        if next_location == location {
            return Ok(location);
        } else {
            return algorithm2(problem, next_location);
        }
    }

    pub fn algorithm3(
        problem: &PeakProblem,
        best_seen: Option<(i32, i32)>,
    ) -> Result<(i32, i32)> {
        if problem.num_row <= 0 || problem.num_col <= 0 {
            return err!("problem {:?} is empty", problem);
        }
        let mid_row = problem.num_row / 2;
        let mid_col = problem.num_col / 2;
        let mut sub_problems = vec![];

        let (sub_start_r1, sub_num_r1) = (0, mid_row);
        let (sub_start_r2, sub_num_r2) = (mid_row + 1, problem.num_row - (mid_row + 1));
        let (sub_start_c1, sub_num_c1) = (0, mid_col);
        let (sub_start_c2, sub_num_c2) = (mid_col + 1, problem.num_col - (mid_col + 1));

        sub_problems.push((sub_start_r1, sub_start_c1, sub_num_r1, sub_num_c1));
        sub_problems.push((sub_start_r1, sub_start_c2, sub_num_r1, sub_num_c2));
        sub_problems.push((sub_start_r2, sub_start_c1, sub_num_r2, sub_num_c1));
        sub_problems.push((sub_start_r2, sub_start_c2, sub_num_r2, sub_num_c2));

        let mut cross = vec![];

        cross.extend(cross_product(
            &vec![mid_row],
            &(0..problem.num_col).collect(),
        ));
        cross.extend(cross_product(
            &(0..problem.num_row).collect(),
            &vec![mid_col],
        ));

        let cross_loc = problem.get_max(cross);
        let neighbor = problem.get_better_neighbor(cross_loc);

        let best_seen_loc = if let Some(best_seen_loc) = best_seen {
            if problem.get(neighbor) > problem.get(best_seen_loc) {
                neighbor
            } else {
                best_seen_loc
            }
        } else {
            neighbor
        };

        println!("{:?}, {:?}, {:?}", cross_loc, neighbor, best_seen_loc);

        if neighbor == cross_loc {
        // if neighbor == cross_loc && problem.get(cross_loc) >= problem.get(best_seen_loc) {
            return Ok(neighbor);
        }

        let sub = problem.get_sub_problem_containing(sub_problems, best_seen_loc);
        let new_best = sub.get_location_in_self(problem, best_seen_loc);
        let result = algorithm3(&sub, Some(new_best))?;
        return Ok(problem.get_location_in_self(&sub, result));
    }

    pub fn algorithm4(
        problem: &PeakProblem,
        best_seen: Option<(i32, i32)>,
        row_split: bool,
    ) -> Result<(i32, i32)> {
        if problem.num_row <= 0 || problem.num_col <= 0 {
            return err!("problem {:?} is empty", problem);
        }

        let mut sub_problems = vec![];
        let divider;

        if row_split {
            let mid = problem.num_row / 2;
            let (sub_start_r1, sub_num_r1) = (0, mid);
            let (sub_start_r2, sub_num_r2) = (mid + 1, problem.num_row - (mid + 1));
            let (sub_start_c, sub_num_c) = (0, problem.num_col);

            sub_problems.push((sub_start_r1, sub_start_c, sub_num_r1, sub_num_c));
            sub_problems.push((sub_start_r2, sub_start_c, sub_num_r2, sub_num_c));

            divider = cross_product(&vec![mid], &(0..problem.num_col).collect());
        } else {
            let mid = problem.num_col / 2;
            let (sub_start_c1, sub_num_c1) = (0, mid);
            let (sub_start_c2, sub_num_c2) = (mid + 1, problem.num_col - (mid + 1));
            let (sub_start_r, sub_num_r) = (0, problem.num_row);

            sub_problems.push((sub_start_r, sub_start_c1, sub_num_r, sub_num_c1));
            sub_problems.push((sub_start_r, sub_start_c2, sub_num_r, sub_num_c2));
            divider = cross_product(&(0..problem.num_row).collect(), &vec![mid]);
        }

        let best_loc = problem.get_max(divider);
        let neighbor = problem.get_better_neighbor(best_loc);

        let best_seen_loc = if let Some(best_seen_loc) = best_seen {
            if problem.get(neighbor) > problem.get(best_seen_loc) {
                neighbor
            } else {
                best_seen_loc
            }
        } else {
            neighbor
        };

        if neighbor == best_loc && problem.get(best_loc) >= problem.get(best_seen_loc) {
            return Ok(best_loc);
        }

        let sub = problem.get_sub_problem_containing(sub_problems, best_seen_loc);
        let new_best = sub.get_location_in_self(problem, best_seen_loc);
        let result = algorithm4(&sub, Some(new_best), !row_split)?;
        return Ok(problem.get_location_in_self(&sub, result));
    }
}
