pub fn my_atoi(s: String) -> i32 {
    let mut result = 0;
    let mut digit = Vec::new();
    let mut positive = true;
    let mut start = false;
    for c in s.chars() {
        if c == ' ' {
            if start {
                break;
            }
            continue;
        }
        if c == '-' {
            if start {
                break;
            }
            positive = false;
            start = true;
            continue;
        }
        if c == '+' {
            if start {
                break;
            }
            positive = true;
            start = true;
            continue;
        }
        if c < '0' || c > '9' {
            break;
        }
        start = true;
        digit.push(c as i32 - '0' as i32);
    }
    for i in 0..digit.len() {
        let max_v = ((i32::MAX as f64) / (digit[i] as f64)).log10();
        if  max_v < (digit.len() - i - 1) as f64 {
            if positive {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }
        let adding = digit[i] * 10_i32.pow((digit.len() - i - 1) as u32);
        if std::i32::MAX - adding < result {
            if positive {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }
        result += adding;
    }
    if positive {
        result
    } else {
       -result 
    }
}