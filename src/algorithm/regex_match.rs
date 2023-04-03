use std::fmt::Display;




pub fn is_match(s: String, p: String) -> bool {
    enum Regex {
        Any,
        Char(char),
        Star(Box<Regex>),
    }

    // impl Display for Regex {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match self {
    //             Regex::Any => write!(f, "."),
    //             Regex::Char(c) => write!(f, "{}", c),
    //             Regex::Star(r) => write!(f, "{}*", r),
    //         }
    //     }
    // }

    let mut regex = vec![];
    for c in p.chars() {
        if c == '*' {
            if let Some(Regex::Star(_)) = regex.last() {
                continue;
            }
            if let Some(r) = regex.pop() {
                regex.push(Regex::Star(Box::new(r)));
            }
        } else if c == '.' {
            if let Some(Regex::Star(v)) = regex.last() {
                let v = v.clone();
                if let Regex::Any = **v {
                    continue;
                }
            }
            regex.push(Regex::Any);
        } else {
            if let Some(Regex::Star(v)) = regex.last() {
                let v = v.clone();
                if let Regex::Char(c2) = **v {
                    if c == c2 {
                        continue;
                    }
                }
            }
            regex.push(Regex::Char(c));
        }
    }
    regex.reverse();
    for (i, c) in s.chars().enumerate() {
        // println!("outer");
        'inner: loop {
            // println!("inner");
            if let Some(r) = regex.pop() {
                match r {
                    Regex::Any => {
                        // println!("c: {} r: {}", c, r);
                        break 'inner;
                    }
                    Regex::Char(c2) => {
                        // println!("c: {} r: {}", c, r);
                        if c != c2 {
                            return false;
                        } else {
                            break 'inner;
                        }
                    }
                    Regex::Star(r) => {
                        // println!("- c: {} r: {}", c, r);
                        match *r {
                            Regex::Any => {
                                if i != s.chars().count() - 1 {
                                    regex.push(Regex::Star(r));
                                }
                                break 'inner;
                            }
                            Regex::Char(c2) => {
                                if c == c2 {
                                    // 多次
                                    if i != s.chars().count() - 1 {
                                        regex.push(Regex::Star(r));
                                    }
                                    break 'inner;
                                } 
                                // 0次 或者最后一次不匹配的情况
                            }
                            Regex::Star(_) => {}
                        }
                    }
                }
            } else {
                return false;
            }
        }
    }
    regex.is_empty()
}