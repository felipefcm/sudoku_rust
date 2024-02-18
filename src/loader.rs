use std::fs;
use std::io;
use std::io::BufRead;

pub fn load_file(path: &str) -> Vec<(String, String)> {
    let mut entries: Vec<(String, String)> = Vec::new();

    let file = fs::File::open(path).expect("Failed to read");
    let reader = io::BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let line = line.expect("Failed to read line");
        let splits: Vec<&str> = line.split(',').collect();

        entries.push((splits[0].to_string(), splits[1].to_string()));
    }

    return entries;
}
