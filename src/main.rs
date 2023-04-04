mod algorithm;

fn main() {
    let v = algorithm::roman_to_int::roman_to_int("III".to_string());
    println!("{}", v);
}
