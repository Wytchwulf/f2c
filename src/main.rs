use std::io::stdin;

fn main() {
    println!("Enter F (Farenheit) or C (Celcius) for unit type to convert");

    let mut unit = String::new();

    stdin().read_line(&mut unit).expect("Unable to read line");

    println!("Enter value to convert");

    let mut val = String::new();

    stdin().read_line(&mut val).expect("Unable to read line");

    let val: f64 = val.trim().parse().expect("Please enter a number");

    if unit.trim().eq_ignore_ascii_case("f") {
        let conversion: f64 = ftoc(val);
        println!("{:.2}°C", conversion);
    } else if unit.trim().eq_ignore_ascii_case("c") {
        let conversion: f64 = ctof(val);
        println!("{:.2}°F", conversion)
    } else {
        println!("Invalid unit type")
    }
}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn ctof(c: f64) -> f64 {
    c * 1.8 + 32.0
}
