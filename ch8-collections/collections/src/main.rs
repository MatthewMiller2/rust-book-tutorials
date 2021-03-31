mod ch1;
mod ch2;

fn main() {
    ch1::mean_median_mode();
    println!("{}", ch2::pig_latin(String::from("first")));
    println!("{}", ch2::pig_latin(String::from("apple")));
}

