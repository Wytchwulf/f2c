use std::io::{stdin};

fn main() {
    println!("Please enter F or C for unit type you wish to convert");

    let mut unit = String::new();

    stdin()
        .read_line(&mut unit)
        .expect("Unable to read line");

    println!("Please enter the value you wish to convert");

    let mut val = String::new();

    stdin()
        .read_line(&mut val)
        .expect("Unable to read line");

    let val: f32 = val.trim().parse().expect("Please enter a number");

    if unit.trim().eq_ignore_ascii_case("f") {
        let conversion = ftoc(val);
        println!("{}", conversion);
    } else {
        let conversion = ctof(val);
        println!("{}", conversion)
    }

}

fn ftoc(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn ctof(c: f32) -> f32 {
    c * 1.8 + 32.0
}
