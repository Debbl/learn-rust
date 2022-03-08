use std::io;
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("神秘数字是: {}", secret_number);
    println!("猜测一个数");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数是：{}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
