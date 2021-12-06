use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let v = read_file("input.in")?;
    let answer = if let Some(a) = measure_depth(v) { a } else { 0 };
    println!("Answer is: {}", answer);
    Ok(())
}

fn measure_depth(v: Vec<usize>) -> Option<usize> {
    let mut acc = 0;
    let mut iter = v.windows(2);
    while let Some(win) = iter.next() {
        if win[0] < win[1] {
            acc += 1;
        }
    }

    if acc > 0 {
        return Some(acc);
    }

    None
}

fn read_file<P>(path: P) -> Result<Vec<usize>>
where
    P: AsRef<Path>,
{
    let mut depths: Vec<usize> = vec![];
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    while let Ok(v) = reader.read_line(&mut buf) {
        if v == 0 {
            break;
        }

        let depth = buf
            .strip_suffix('\n')
            .unwrap()
            .parse::<usize>()
            .expect("value cannot be converted to number");
        depths.push(depth);
        buf.clear();
    }

    Ok(depths)
}

#[test]
fn test_empty() {
    let v: Vec<usize> = vec![];
    assert_eq!(None, measure_depth(v));
}

#[test]
fn test_one() {
    let v: Vec<usize> = vec![1];
    assert_eq!(None, measure_depth(v));
}

#[test]
fn test_two() {
    let v: Vec<usize> = vec![1, 2];
    assert_eq!(Some(1), measure_depth(v));
}

#[test]
fn test_two_identical() {
    let v: Vec<usize> = vec![1, 1];
    assert_eq!(None, measure_depth(v));
}

#[test]
fn test_many_wide() {
    let v: Vec<usize> = vec![1, 2, 1, 2, 3, 4, 2, 1];
    assert_eq!(Some(4), measure_depth(v));
}

#[test]
fn test_e2e() {
    let v = read_file("test-input.in").unwrap();
    assert_eq!(Some(7), measure_depth(v));
}
