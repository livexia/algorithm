use dna_seq_rust::dnaseqlib;

use std::env;
use std::error::Error;
use std::io;
use std::result;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} [file_a.fa] [file_b.fa] [output.png]", args[0]);
    } else {
        dnaseqlib::compare_sequences(&args[3], (500, 500), &args[1], &args[2], 8, 100)?;
    }
    Ok(())
}
