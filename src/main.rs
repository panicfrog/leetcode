mod algorithm;

fn main() {
    let r = algorithm::longest_palindrome::longest_palindrome("bb".to_string());
    println!("{}", r);
}
