fn main() {
    // x = 6;
    // println!("This value of x is: {}", x);

    // let _y = 1;

    // let (a, mut b) = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // const MAX_POINTS: u32 = 100_000;
    // println!("{}", MAX_POINTS)

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("spaces len is: {}", spaces);

    // 0000 0010
    // 0000 0011
    let a = 2;
    let b = 3i64;
    let x = -4;

    println!("{}", a & b);
    println!("{}", !b);
    println!("{}", b >> 1);
    println!("{}", x);

    for i in 1..=5 {
        println!("{}", i);
    }
    for a in 'a'..='z' {
        println!("{}", a);
    }
}
