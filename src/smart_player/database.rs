use std::fs::OpenOptions;
use std::io::prelude::*;

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

pub fn save_seq(filename: &str, seq: &[i32]) {
    let path = format!("db/{}-{}.txt", &seq.len().to_string(), filename);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Nyak");

    writeln!(file, "{}", seq_to_string(seq)).expect("Nyek");
}
