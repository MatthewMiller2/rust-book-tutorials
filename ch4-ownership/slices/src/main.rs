fn main() {
    // Slices using Strings
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("word: {}", word);
    let word = first_word(&my_string);
    println!("word: {}", word);
    
    // Slices using str literals
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("word: {}", word);
    let word = first_word(my_string_literal);
    println!("word: {}", word);

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    let slice = &arr[1..3];
    println!("slice: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
