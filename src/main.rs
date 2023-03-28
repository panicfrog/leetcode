mod algorithm;

fn main() {
    let convert = algorithm::n_convert::convert(String::from("A"), 1);
    println!("{}", convert);
}
