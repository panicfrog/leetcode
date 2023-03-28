
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
        let max_v = ((i32::MAX as f64) / (digit[i] as f64)).log10();
        if  max_v < (digit.len() - i - 1) as f64 {
            return 0;
        }
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