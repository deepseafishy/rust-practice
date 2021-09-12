enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    /*
      Enums
    */
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    /*
      Option enum
    */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = Option::None;

    //Option<T> and T are different
    let x: i8 = 5;
    let y: Option<i8> = Option::Some(5);
    //let sum = x + y; //This will cause an error

    /*
      match control flow operator
    */
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    /*
      _ placeholder
    */
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    /*
      if let
    */
    let some_u8_value = Option::Some(0u8);
    if let Option::Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //Without all cases, the compiler will return an error
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}
