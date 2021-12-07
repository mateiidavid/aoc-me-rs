use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct Course {
    depth: usize,
    horizontal: usize,
}

impl Course {
    fn new() -> Self {
        Course {
            depth: 0,
            horizontal: 0,
        }
    }

    fn mv(&mut self, mve: &Move) {
        match mve {
            Move::Forward(v) => self.horizontal += v,
            Move::Down(v) => self.depth += v,
            Move::Up(v) => self.depth -= v,
        }
    }

    fn finish(&self) -> usize {
        &self.horizontal * &self.depth
    }
}

#[derive(Debug)]
enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl TryFrom<String> for Move {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        let mv = split.next();
        let val = split.next();
        if mv.is_none() || val.is_none() {
            return Err("invalid move");
        };

        let val = val.unwrap().parse::<usize>();
        if let Err(_) = val {
            return Err("could not convert argument to number");
        };

        match mv {
            Some(s) if s.to_lowercase() == "forward" => Ok(Move::Forward(val.unwrap())),
            Some(s) if s.to_lowercase() == "down" => Ok(Move::Down(val.unwrap())),
            Some(s) if s.to_lowercase() == "up" => Ok(Move::Up(val.unwrap())),
            _ => Err("invalid move"),
        }
    }
}

fn chart_course(c: &mut Course, move_list: Vec<Move>) -> usize {
    move_list.iter().for_each(|m| c.mv(m));
    c.finish()
}

fn read_file<P>(path: P) -> std::io::Result<Vec<Move>>
where
    P: AsRef<Path>,
{
    let mut moves: Vec<Move> = vec![];
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    while let Ok(v) = reader.read_line(&mut buf) {
        if v == 0 {
            break;
        }

        let mv = buf.clone().try_into().unwrap();
        moves.push(mv);
        buf.clear();
    }

    Ok(moves)
}
fn main() -> std::io::Result<()> {
    let v = read_file("day2.in")?;
    let mut c = Course::new();
    v.iter().for_each(|m| c.mv(m));

    println!("Result: {}", c.finish());

    Ok(())
}

#[test]
fn test_aoc_case() {
    let v: Vec<Move> = vec![
        Move::Forward(5),
        Move::Down(5),
        Move::Forward(8),
        Move::Up(3),
        Move::Down(8),
        Move::Forward(2),
    ];

    let mut c = Course::new();
    assert_eq!(150, chart_course(&mut c, v))
}
