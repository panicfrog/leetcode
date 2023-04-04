pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut chars = s.chars();
    let mut prev = ' ';
    while let Some(c) = chars.next() {
        match c {
            'I' => {
                result += 1;
            }
            'V' => {
                if prev == 'I' {
                    result += 3;
                } else {
                    result += 5;
                }
            }
            'X' => {
                if prev == 'I' {
                    result += 8;
                } else {
                    result += 10;
                }
            }
            'L' => {
                if prev == 'X' {
                    result += 30;
                } else {
                    result += 50;
                }
            }
            'C' => {
                if prev == 'X' {
                    result += 80;
                } else {
                    result += 100;
                }
            }
            'D' => {
                if prev == 'C' {
                    result += 300;
                } else {
                    result += 500;
                }
            }
            'M' => {
                if prev == 'C' {
                    result += 800;
                } else {
                    result += 1000;
                }
            }
            _ => {}
        }
        prev = c;
    }
    result
}