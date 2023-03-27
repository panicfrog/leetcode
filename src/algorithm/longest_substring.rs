use std::collections::HashMap;

// 最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left: usize = 0;
    let mut max: usize = 0;
    let mut index_map:HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    for right in 0..chars.len() {
        let right_char = &chars[right];
        if index_map.contains_key(right_char) && index_map[&right_char] >= left {
            left = index_map[&right_char] + 1;
        }
        index_map.insert(*right_char, right);
        let cur_len = right - left + 1;
        max = cur_len.max(max);
    }
    return max as i32;

}