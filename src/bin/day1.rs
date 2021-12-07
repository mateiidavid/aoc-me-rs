use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let v = read_file("input.in")?;
    let answer = if let Some(a) = measure_depth(v.clone()) {
        a
    } else {
        0
    };
    println!("Answer is: {}", answer);

    let win_answer = if let Some(a) = measure_depth_win(v.clone()) {
        a
    } else {
        0
    };

    println!("Answer for window sliding is: {}", win_answer);

    let classic = if let Some(a) = measure_depth_classic(v) {
        a
    } else {
        0
    };
    println!("Answer for window sliding done differently is: {}", classic);
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

fn measure_depth_win(v: Vec<usize>) -> Option<usize> {
    let mut acc = 0;
    let mut windows = v.windows(4);
    // Can keep a prev value if I want to
    // only work on a sliding win of 3
    while let Some(win) = windows.next() {
        let first: usize = win[..3].into_iter().sum();
        let second: usize = win[1..].into_iter().sum();
        if first < second {
            acc += 1;
        }
    }

    if acc > 0 {
        return Some(acc);
    }

    None
}

fn measure_depth_classic(v: Vec<usize>) -> Option<usize> {
    if v.len() < 2 {
        return None;
    }

    let mut accumulator = 0;
    let mut index = 0;

    while let Some(_) = v.iter().next() {
        if index + 3 >= v.len() {
            break;
        }
        let first: usize = v[index..index + 3].iter().sum();
        let second: usize = v[index + 1..index + 4].iter().sum();
        if first < second {
            accumulator += 1;
        }

        index += 1;
    }

    if accumulator == 0 {
        return None;
    }

    Some(accumulator)
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

#[test]
fn test_win() {
    let v: Vec<usize> = vec![607, 618, 618, 617, 647, 716, 769, 792];
    assert_eq!(Some(5), measure_depth_win(v));
}
