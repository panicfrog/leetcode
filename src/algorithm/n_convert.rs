pub fn convert(s: String, num_rows: i32) -> String {
    let chars = s.chars().collect::<Vec<char>>();
    let mut result = String::new();
    let num_rows = num_rows as usize;
    let mut step: usize = 0;
    // step 等于0: 0, 4, 8, 12 ...
    // step 等于1: 1, 3, 5, 7 ... 
    // step 等于2: 2, 6, 10, 14 ...

    if num_rows == 1 {
        return s;
    }
    while step < num_rows {
        if step == 0 || step == num_rows - 1 {
            // 头尾行
            // 间隔 2 * (num_rows - 1)
            // 在索引没有超过字符串总长度的时候一次加入到结果中
            let mut index = step;
            while index < chars.len() {
                result.push(chars[index]);
                index += 2 * (num_rows - 1);
            }
        } else {
            // 中间行
            // 第一个为
            let mut n: usize = 0;
            let mut index: usize = step;
            while index < chars.len() {
                // 间隔: 
                // 偶数向下 奇数向上
                let margin = if n % 2 == 0 { 
                    2 * (num_rows - 1 - step) 
                } else { 
                    2 * step
                };
                result.push(chars[index]);
                index += margin;
                n += 1;
            }
        }
        step += 1;
    }
    return result;
}