
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();
    let mut result = String::new();
    for (i, c) in shortest.chars().enumerate() {
        if strs.iter().all(|s| s.chars().nth(i).unwrap() == c) {
            result.push(c);
        } else {
            break;
        }
    }
    result
}