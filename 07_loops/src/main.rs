/* fn main() {
    loop {
        println!("again!");
    }
}
 */

/* fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
} */

/* fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
} */

/* fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF");
} */

/* fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // println!("\n");
    for e in a {
        println!("the value is: {e}");
    }
}
 */

/* fn main() {
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("LIFTOFF");
} */

fn main() {
    for i in 0..=10 {
        println!("-------{i}------");
        println!("{}", fibonacci(i));
        println!("{}", fibonacci_2(i));
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn fibonacci_2(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }
    let mut a;
    let mut b = 1;
    let mut c = 1;
    for _ in 3..=n {
        a = b;
        b = c;
        c = a + b;
    }
    c
}
