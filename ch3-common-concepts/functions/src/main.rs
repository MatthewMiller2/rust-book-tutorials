fn main() {
    another_function(5, 6);
    let x = five();
    println!("The value of x is {}", x);
    let y = plus_one(5);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("{}", x);
    println!("{}", y);
}
