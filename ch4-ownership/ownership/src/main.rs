fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

// gives_ownership will move its return value into
// the function that calls it
fn gives_ownership() -> String { 
    // some_string comes into scope
    let some_string = String::from("hello");
    
    // some_string is returned and moves out to the
    // calling function
    some_string
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}
