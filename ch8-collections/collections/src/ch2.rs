
pub fn pig_latin(s: String) -> String {
    let first_char = &s[0..1];
    let mut ret: String;
    let suffix = match first_char {
        "a" | "e" | "i" | "o" | "u" => {
            ret = s.clone();
            String::from("hay")
        },
        _ => {
            ret = String::from(&s[1..]);
            let mut s = String::from(first_char);
            s.push_str("ay");
            s
        },
    };
    ret.push_str(&suffix);
    ret
}
