
// static lifetime in struct data
struct LavaLamp {
    colours: &'static str
}

// string literal
fn str_literal() -> &'static str {
    "I'm a string literal"
}

// named lifetime
struct Mobile<'m> {
    brand1: &'m str,
    brand2: &'m str,
    brand3: &'m str,
}

// implementation with anonymous lifetime (from Mobile)
impl Mobile<'_> {
    fn show_mobile(&self) {
        println!("{} {} {}", self.brand1, self.brand2, self.brand3);
    }
}

fn main() {
    let a = LavaLamp {
        // again needs to be literal
        colours: "blue, green, red"
    };
    println!("{}", a.colours);

    let b = str_literal();
    println!("{}", b);

    let c_vec = vec!["LG".to_string(), "Apple".to_string(), "Huawei".to_string()];
    let c = Mobile {
        brand1: &c_vec[0],
        brand2: &c_vec[1],
        brand3: &c_vec[2],
    };
    println!("{} {} {}", c.brand1, c.brand2, c.brand3);

    let d = Mobile {
        brand1: &c_vec[0],
        brand2: &c_vec[1],
        brand3: &c_vec[2],
    };
    d.show_mobile();

}
