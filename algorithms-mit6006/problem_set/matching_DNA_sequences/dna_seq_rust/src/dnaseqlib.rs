use bio::io::fasta;
use std::error::Error;
use std::result;
use image::{Rgb, RgbImage};

use crate::ExactSubMatchesIterator;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

type Result<T> = result::Result<T, Box<dyn Error>>;

pub struct RollingHash {
    hash_base: u64,
    seqlen: u32,
    curhash: u64,
}

impl RollingHash {
    pub fn new(s: &[u8]) -> Self {
        let hash_base: u64 = 7;
        let seqlen = s.len() as u32;
        let mut n = seqlen - 1;
        let mut h = 0;
        for &c in s {
            h += c as u64 * (hash_base.pow(n));
            n = n.saturating_sub(1);
        }
        let curhash = h;
        Self {
            hash_base,
            seqlen,
            curhash,
        }
    }

    pub fn current_hash(&self) -> u64 {
        self.curhash
    }

    pub fn slide(&mut self, prev_item: u8, next_item: u8) -> u64 {
        self.curhash = (self.curhash * self.hash_base) + next_item as u64;
        self.curhash -= prev_item as u64 * (self.hash_base.pow(self.seqlen));
        self.curhash
    }
}

fn build_comprison_image(
    filename: &str,
    w: usize,
    h: usize,
    a_len: usize,
    b_len: usize,
    matches: ExactSubMatchesIterator,
) -> Result<()> {
    let mut arr = vec![vec![0u64; w]; h];
    println!("Sequence A length: {}", a_len);
    println!("Sequence B length: {}", b_len);
    let a_bin_size = (a_len as f64 / w as f64).ceil() as usize;
    let b_bin_size = (b_len as f64 / w as f64).ceil() as usize;
    assert!(a_bin_size > 0);
    assert!(b_bin_size > 0);
    println!("Binning matches...");
    for m_v in matches {
        for m in m_v {
            arr[m.0 / a_bin_size][m.1/ b_bin_size] += 1;
        }
    }
    println!("...done binning matches.");
    println!("Normalizing and plotting results...");
    let max = *arr.iter().flatten().max().unwrap() as f64;
    let mut img = RgbImage::new(w as u32, h as u32);
    for y in 0..h {
        for x in 0..w {
            let val = 255 - ((arr[y][x] as f64 / max).sqrt().sqrt() * 255.0).ceil() as u8;
            img.put_pixel(y as u32, x as u32, Rgb([val, val, val]))
        }
    }
    println!("...done normalizing and plotting.");
    img.save(filename)?;
    Ok(())
}

pub fn compare_sequences(
    img_file: &str,
    img_size: (usize, usize),
    a_file: &str,
    b_file: &str,
    k: usize,
    m: usize,
) -> Result<()> {
    let a = fasta::Reader::from_file(a_file)?;
    let b = fasta::Reader::from_file(b_file)?;

    let a = a.records().next().unwrap()?;
    let a_len = a.seq().len();
    let mut a_bytes = a.seq().iter();

    let b = b.records().next().unwrap()?;
    let b_len = b.seq().len();
    let mut b_bytes = b.seq().iter();

    let mut matches = ExactSubMatchesIterator::new(&mut a_bytes, &mut b_bytes, k, m);
    build_comprison_image(img_file, img_size.0, img_size.1, a_len, b_len, matches)?;
    Ok(())
}
