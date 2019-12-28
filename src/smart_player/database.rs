use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader};

fn seq_to_string(seq: &[i32]) -> String {
    if seq.len() == 0 {
        return String::new();
    }

    let mut ret = seq[0].to_string();

    for i in &seq[1..] {
        ret = ret + "," + &i.to_string();
    }

    return ret;
}

fn string_to_seq<'a>(s: String) -> Vec<i32> {
    s.split(",")
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect()
}

pub struct Database {
    file: Option<BufReader<std::fs::File>>,
}

impl Database {
    pub fn save_seq(filename: &str, seq: &[i32]) {
        let path = format!("db/{}-{}.txt", &seq.len().to_string(), filename);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .expect("Nyak");

        writeln!(file, "{}", seq_to_string(seq)).expect("Nyek");
    }

    pub fn read_seq(filename: &str, len: usize) -> Database {
        let path = format!("db/{}-{}.txt", len.to_string(), filename);

        Database {
            file: match OpenOptions::new().read(true).open(path) {
                Err(err) => match err.kind() {
                    std::io::ErrorKind::NotFound => None,
                    kind => panic!(kind),
                },
                Ok(file) => Some(BufReader::new(file)),
            },
        }
    }
}

impl Iterator for Database {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        match &mut self.file {
            None => None,
            Some(f) => f
                .lines()
                .filter_map(|line| line.ok())
                .map(|line| string_to_seq(line))
                .next(),
        }
    }
}
