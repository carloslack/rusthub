/**
 * Create, write and read a small file
 */

use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::prelude::*;
use std::fs;

static HOSTS_STR: &str =
"127.0.0.1 localhost
192.160.0.2 hostA.com
192.168.0.3 hostB.com
";

fn show(hosts_file: &str) {
    let mut buf = String::new();
    let mut f = File::open(hosts_file).expect("open error");

    f.read_to_string(&mut buf).expect("read_to_string error");
    print!("{}",buf);
}

fn main() {
    let hosts_file: &str = "/tmp/hosts";
    let p = Path::new(hosts_file);
    let mut f = match File::create(&p) {
        Err(why) => panic!("failed to create {}: {}", hosts_file, why),
        Ok(file) =>file
    };

    match f.write_all(HOSTS_STR.as_bytes()) {
        Err(why) => panic!("failed to write {}: {}", hosts_file, why),
        Ok(_) => println!("wrote to {}", hosts_file)
    }

    show(hosts_file);
    fs::remove_file(hosts_file).expect("error");
}

