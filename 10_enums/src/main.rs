/* enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    route(six);
    println!("Hello, world!");
}

fn route(ip_kind: IpAddrKind) {} */

/* enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
} */

/* enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColor(i32, i32, i32);

impl Message {
    fn call(&self) {}
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Quit;
    m.call();
} */

/* fn main() {
    let some_number = Option::Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + match y {
        Some(x) => x,
        None => match Some(5) {
            _ => 0,
        },
    };
    let sum_1 = x + y.unwrap();
    println!("{}", sum);
} */

/* fn main() {
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
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny");
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
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(1),
        o => remove_fancy_hat(o),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(1),
        _ => (),
    }
}

fn remove_fancy_hat(i: i32) {
    todo!()
}

fn add_fancy_hat() {
    todo!()
}
 */

fn main() {
    let mut config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    config_max = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("else");
    }
}
