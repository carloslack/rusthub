/**
 * Simple trait
 */
struct Stat  {
    inode: i32,
    access: i32,
    size: i32,
    path: String,
    format: String,
}

trait About {
    fn summary(&self) -> String;
}

impl About for Stat {
    fn summary(&self) -> String {
        format!("inode:{}, access:{}, size:{}, path:{}, format:{}",
                self.inode, self.access, self.size, self.path, self.format)
    }
}


fn main() {

    let stat = Stat {
        inode: 12345,
        access: 0644,
        size: 175,
        path: String::from("/tmp/bin"),
        format: String::from("ELF"),
    };

    println!("{}", stat.summary());
}
