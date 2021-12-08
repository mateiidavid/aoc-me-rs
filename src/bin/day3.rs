use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn dg_power_report(v: &mut Vec<String>) -> usize {
    let mut gamma = String::new();
    let mut eps = String::new();
    let mut numbers = v.iter_mut();
    let mut finished = false;
    let mut index = 0;
    let mut freq = [0, 0];
    loop {
        while let Some(number) = numbers.next() {
            let digit = number.chars().nth(index);
            if digit.is_none() {
                finished = true;
                break;
            }
            let val = digit.unwrap().to_digit(2).unwrap();
            freq[val as usize] += 1;
        }

        if finished {
            break;
        }

        // Get max value
        let mut gamma_value = 0;
        let mut eps_value = 0;
        if freq[0] < freq[1] {
            gamma_value = 1;
        } else {
            eps_value = 1;
        }
        freq[0] = 0;
        freq[1] = 0;
        gamma.push(char::from_digit(gamma_value, 10).unwrap());
        eps.push(char::from_digit(eps_value, 10).unwrap());

        index += 1;
        numbers = v.iter_mut();
    }

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let eps = usize::from_str_radix(&eps, 2).unwrap();

    return gamma * eps;
}

fn read_file<P>(path: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut bits: Vec<String> = vec![];
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    while let Ok(v) = reader.read_line(&mut buf) {
        if v == 0 {
            break;
        }

        let bit = buf.strip_suffix('\n').unwrap();
        bits.push(bit.to_string());
        buf.clear();
    }

    Ok(bits)
}
fn main() {
    let mut v = read_file("day3.in").unwrap();
    println!("{}", dg_report(&mut v));
}

#[test]
fn test_loop_basic() {
    let mut v: Vec<String> = vec![
        String::from("00100"),
        String::from("11110"),
        String::from("10110"),
        String::from("10111"),
        String::from("10101"),
        String::from("01111"),
        String::from("00111"),
        String::from("11100"),
        String::from("10000"),
        String::from("11001"),
        String::from("00010"),
        String::from("01010"),
    ];
    assert_eq!(198, dg_report(&mut v));
}
