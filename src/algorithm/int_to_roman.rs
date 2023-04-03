pub fn int_to_roman(num: i32) -> String {
   // 总共情况有 13 种
    // 1. 1-3
    // 2. 4
    // 3. 5 - 8
    // 4. 9
    // 5. 10 - 30
    // 6. 40
    // 7. 50 - 80
    // 8. 90
    // 9. 100 - 300
    // 10. 400
    // 11. 500 - 800
    // 12. 900
    // 13. 1000 - 3000 
    let mut num = num;
    let mut result = String::new();
    while num > 0 {
        if num >= 1000 {
            result.push_str("M".repeat((num / 1000) as usize).as_str());
            num %= 1000;
        } else if num >= 900 {
            result.push_str("CM");
            num -= 900;
        } else if num >= 500 {
            result.push_str("D");
            num -= 500;
        } else if num >= 400 {
            result.push_str("CD");
            num -= 400;
        } else if num >= 100 {
            result.push_str("C".repeat((num / 100) as usize).as_str());
            num %= 100;
        } else if num >= 90 {
            result.push_str("XC");
            num -= 90;
        } else if num >= 50 {
            result.push_str("L");
            num -= 50;
        } else if num >= 40 {
            result.push_str("XL");
            num -= 40;
        } else if num >= 10 {
            result.push_str("X".repeat((num / 10) as usize).as_str());
            num %= 10;
        } else if num >= 9 {
            result.push_str("IX");
            num -= 9;
        } else if num >= 5 {
            result.push_str("V");
            num -= 5;
        } else if num >= 4 {
            result.push_str("IV");
            num -= 4;
        } else if num >= 1 {
            result.push_str("I".repeat(num as usize).as_str());
            num = 0;
        }
    } 
    result
}