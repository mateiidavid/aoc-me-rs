use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_bit_frequencies(numbers: &Vec<String>) -> Vec<[u32; 2]> {
    let mut freq: Vec<[u32; 2]> = vec![[0; 2]; 12];
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

    freq
}

fn get_common_bits(numbers: &Vec<String>) -> Vec<u32> {
    let freq = get_bit_frequencies(numbers);
    let mut iter = freq.into_iter();
    let mut common = vec![];
    while let Some(pair) = iter.next() {
        let (a, b) = (pair[0], pair[1]);
        if a > b {
            common.push(0);
        } else {
            common.push(1);
        }
    }

    common
}

fn get_common_bit(numbers: &Vec<String>) -> char {
    let freq = get_bit_frequencies(numbers);
    let mut iter = freq.into_iter();
    let common = if let Some(pair) = iter.next() {
        let (a, b) = (pair[0], pair[1]);
        if a > b {
            '0'
        } else {
            '1'
        }
    } else {
        '1'
    };

    common
}

fn get_least_common(numbers: &Vec<String>) -> char {
    let common = get_common_bit(numbers);
    let least = if common == '0' { '1' } else { '0' };
    least
}

fn get_least_common_bits(bits: Vec<u32>) -> Vec<u32> {
    bits.into_iter()
        .map(|d| if d == 0 { 1 } else { 0 })
        .collect()
}

fn convert_to_str(number: Vec<u32>) -> String {
    number
        .into_iter()
        .map(|d| if d == 0 { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("")
}

fn remove_one(numbers: Vec<(String, String)>, bit: char) -> Vec<(String, String)> {
    numbers
        .into_iter()
        .filter_map(|(preserved, number)| {
            // get char
            // if char good => slice
            match number.chars().nth(0) {
                Some(c) => {
                    if c == bit {
                        Some((preserved, number[1..].to_string()))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect::<Vec<(String, String)>>()
}

fn calculate_oxygen_rating(numbers: Vec<(String, String)>, bit: char) -> Vec<(String, String)> {
    if numbers.len() == 1 {
        return numbers;
    }

    let numbers = remove_one(numbers, bit);

    let bit = {
        let flat_numbers = numbers
            .iter()
            .map(|(_, number)| number.to_owned())
            .collect();
        get_common_bit(&flat_numbers)
    };

    return calculate_oxygen_rating(numbers, bit);
}

fn calculate_co2_rating(numbers: Vec<(String, String)>, bit: char) -> Vec<(String, String)> {
    if numbers.len() == 1 {
        return numbers;
    }

    let numbers = remove_one(numbers, bit);

    let bit = {
        let flat_numbers = numbers
            .iter()
            .map(|(_, number)| number.to_owned())
            .collect();
        get_least_common(&flat_numbers)
    };

    return calculate_co2_rating(numbers, bit);
}

fn calculate_life_support(numbers: Vec<String>) -> Result<usize> {
    // start w/ full list of numbers
    // keep only numbers selected by the bit criteria
    // only one number left? stop, this is the number
    //
    // oxy: most common bit in the current position

    let tuple_numbers = numbers
        .clone()
        .into_iter()
        .map(|number| (number.clone(), number))
        .collect::<Vec<(String, String)>>();
    let common = get_common_bit(&numbers);
    println!("common: {:?}", common);
    let oxygen_number = {
        let numbers = calculate_oxygen_rating(tuple_numbers.clone(), common);
        let number = match numbers.get(0) {
            Some((preserved, _)) => preserved,
            None => return Err("damn".into()),
        };
        println!("{} with len {}", number, numbers.len());
        usize::from_str_radix(number, 2)?
    };

    let least = get_least_common(&numbers);
    let co2_number = {
        let numbers = calculate_co2_rating(tuple_numbers, least);
        let number = match numbers.get(0) {
            Some((preserved, _)) => preserved,
            None => return Err("damn son".into()),
        };
        usize::from_str_radix(number, 2)?
    };

    println!("{} and {}", oxygen_number, co2_number);
    Ok(oxygen_number * co2_number)
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
    println!("{}", calculate_diagnostics(v.clone())?);
    println!("{}", calculate_life_support(v)?);
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

#[test]
fn test_life_support() {
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
    assert_eq!(230, calculate_life_support(v).unwrap());
}
