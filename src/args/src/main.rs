/*
 * Playing with arguments, bitwise and match
 */

use std::env;

struct ArgOpts {
    a:i8, b:i8, c:i8, d:i8, e:i8, f:i8
}

fn m(v: bool, n: &str) {
    match v {
        false => println!("{}: false", n),
        true => println!("{}: true", n),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = ArgOpts{ a:1, b:2, c:4, d:8, e:16, f:32 };
    let mut flags = 0i8;

    for n in 1..args.len() {
        if args[n].eq("all") {
            flags |= data.a | data.b |
                data.c | data.d | data.e |data.f;
            break;
        }
        if args[n].eq("a") {
            flags |= data.a;
        }
        if args[n].eq("b"){
            flags |= data.b;
        } if args[n].eq("c"){
            flags |= data.c;
        } if args[n].eq("d"){
            flags |= data.d;
        } if args[n].eq("e"){
            flags |= data.e;
        } if args[n].eq("f"){
            flags |= data.f;
        }
    }

    m((flags & 1) != 0, "a");
    m((flags & 2) != 0, "b");
    m((flags & 4) != 0, "c");
    m((flags & 8) != 0, "d");
    m((flags & 16) != 0, "e");
    m((flags & 32) != 0, "f");

}
