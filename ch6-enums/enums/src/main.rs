#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}, None: {:?}", six, none);

    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => ()
    }

    if let Some(3) = some_u8_value {
        println!("three as well!");
    } else {
        ()
    }
}
