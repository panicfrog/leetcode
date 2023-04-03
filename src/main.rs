mod algorithm;

fn main() {
   let i = algorithm::regex_match::is_match("aa".to_string(), "a*".to_string());
    println!("{}", i);
}
