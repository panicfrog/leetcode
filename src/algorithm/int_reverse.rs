
pub fn reverse(x: i32) -> i32 {
    let positive = x > 0;
    let mut x = x.abs();
    let mut result = 0;
    let mut digit = Vec::new();
    while x >= 10  {
        digit.push(x % 10);
        x = x / 10;
    }
    digit.push(x);
    for i in 0..digit.len() {
        let adding = digit[i] * 10_i32.pow((digit.len() - i - 1) as u32);
        if std::i32::MAX - adding < result {
            return 0;
        }
        result += adding;
    }
    if positive {
        result
    } else {
       -result 
    }
}