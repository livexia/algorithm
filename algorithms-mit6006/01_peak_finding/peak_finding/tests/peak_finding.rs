use std::error::Error;
use std::fs;
use std::result;

use peak_finding::peak::{self, PeakProblem};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

fn load_problem(file: &str) -> Result<PeakProblem> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut array = vec![];
    for line in contents.lines() {
        array.push(
            line.trim()
                .split(",")
                .map(|s| {
                    s.trim()
                        .parse()
                        .expect(&format!("parse i32 error: {:?}", s))
                })
                .collect(),
        )
    }
    Ok(PeakProblem::create_problem(array))
}

#[test]
fn test_algorithm1() -> Result<()> {
    let problem = load_problem("./tests/problem.txt")?;
    let peak = peak::algorithm1(&problem)?;
    assert!(
        problem.is_peak(peak),
        "{:?} is NOT a peak (INCORRECT!)",
        peak
    );
    println!("alogrithm1: {:?} is a peak", peak);
    Ok(())
}

#[test]
fn test_algorithm2() -> Result<()> {
    let problem = load_problem("./tests/problem.txt")?;
    let peak = peak::algorithm2(&problem, (0, 0))?;
    assert!(
        problem.is_peak(peak),
        "{:?} is NOT a peak (INCORRECT!)",
        peak
    );
    println!("alogrithm2: {:?} is a peak", peak);
    Ok(())
}

#[test]
fn test_algorithm3() -> Result<()> {
    let problem = load_problem("./tests/problem.txt")?;
    let peak = peak::algorithm3(&problem, None)?;
    assert!(
        problem.is_peak(peak),
        "{:?} is NOT a peak (INCORRECT!)",
        peak
    );
    println!("alogrithm3: {:?} is a peak", peak);
    Ok(())
}

#[test]
fn test_algorithm4() -> Result<()> {
    let problem = load_problem("./tests/problem.txt")?;
    let peak = peak::algorithm4(&problem, None, true)?;
    assert!(
        problem.is_peak(peak),
        "{:?} is NOT a peak (INCORRECT!)",
        peak
    );
    println!("alogrithm4: {:?} is a peak", peak);
    Ok(())
}
