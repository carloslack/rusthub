/**
 * Just search for some strings and return
 * total length for each run
 */
use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn search_some_string(f: &str, m: &str, tot: &mut i32) -> io::Result<()> {
    *tot = 0;
    for line in BufReader::new(File::open(f)?).lines() {
        let line = line?;
        let len = line.len();
        if line.starts_with(m) {
            *tot = *tot + len as i32;

            print!("{} ", line);
        }
    }
    Ok(())
}

fn main() {
    let f: &str = "/proc/meminfo";
    let mut l: i32 = 0;

    if let Err(e) = search_some_string(f, "MemTotal: ", &mut l) {
        eprintln!("error {}", e);
    }
    println!("(len {})", l);

    _ = search_some_string(f, "MemFree: ", &mut l);
    println!("(len {})", l);

    _ = search_some_string(f, "MemAvailable: ", &mut l);
    println!("(len {})", l);
}

