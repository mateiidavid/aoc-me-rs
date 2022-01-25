use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_common_bits(numbers: &Vec<String>) -> Vec<u8> {
    let mut freq: Vec<[u8; 2]> = vec![[0; 2]; 5];
    let mut numbers = numbers.iter();
    while let Some(number) = numbers.next() {
        let mut i = 0;
        number
            .chars()
            .map(|c| c.to_digit(2))
            .filter_map(|d| d)
            .for_each(|d| {
                if d == 0 {
                    freq[i][0] += 1;
                } else if d == 1 {
                    freq[i][1] += 1;
                }
                i += 1;
            });
    }

    let mut iter = freq.into_iter();
    let mut common = vec![];
    while let Some(pair) = iter.next() {
        let (a, b) = (pair[0], pair[1]);
        if a < b {
            common.push(1);
        } else {
            common.push(0);
        }
    }

    common
}

fn get_least_common_bits(frequencies: Vec<u8>) -> Vec<u8> {
    frequencies
        .into_iter()
        .map(|d| if d == 0 { 1 } else { 0 })
        .collect()
}

fn convert_to_str(number: Vec<u8>) -> String {
    number
        .into_iter()
        .map(|d| if d == 0 { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("")
}

fn calculate_diagnostics(numbers: Vec<String>) -> Result<usize> {
    let most_common = get_common_bits(&numbers);
    let least_common = get_least_common_bits(most_common.clone());
    let gamma = {
        let gamma = convert_to_str(most_common);
        usize::from_str_radix(&gamma, 2)?
    };
    let epsilon = {
        let eps = convert_to_str(least_common);
        usize::from_str_radix(&eps, 2)?
    };

    Ok(gamma * epsilon)
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

fn main() -> Result<()> {
    let v = read_file("day3.in").unwrap();
    println!("{}", calculate_diagnostics(v)?);
    Ok(())
}

#[test]
fn test_loop_basic() {
    let v: Vec<String> = vec![
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
    assert_eq!(198, calculate_diagnostics(v).unwrap());
}
