
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut max_lenght = vec![1; chars.len()];
    let mut max_index = 0;
    for i in 0..chars.len() - 1 {
        // 以i为中心
        // 判断i是否大于0，如果大于0，说明i-1是合法的
        if i > 0 {
            let mut left = i;
            let mut right = i;
            while left >= 1 && right < chars.len() - 1 && chars[left - 1] == chars[right + 1] {
                left -= 1;
                right += 1;
            }
            if right - left + 1 > max_lenght[max_index] {
                max_index = i;
                max_lenght[max_index] = right - left + 1;
            }
        }

        // 以i和i+1为中心
        let mut left = i;
        let mut right = i + 1;
        if chars[left] != chars[right] {
            continue;
        }
        while left >= 1 && right < chars.len() - 1 && chars[left - 1] == chars[right + 1] {
            left -= 1;
            right += 1;
        }
        if right - left + 1 > max_lenght[max_index] {
            max_index = i;
            max_lenght[max_index] = right - left + 1;
        }
    }
    let start = max_index - (max_lenght[max_index] - 1) / 2;
    let end = max_index + max_lenght[max_index] / 2;
    return String::from(&s[start..=end]);
}
