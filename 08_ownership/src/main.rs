/* fn main() {
    let mut s = "hello";
    {
        s = "aaa";
        println!("{s}");
        let s = "world";
        println!("{s}");
    }
    println!("{s}");
} */

/* fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");
    println!("{s}");
} */

/* fn main() {
    {
        let s = String::from("hello");
        // s.push_str(", world");
    }
    println!("{s}"); // error
} */

/* fn main() {
    let mut x = 5;
    let y = x;
    x += 1;
    println!("{x} - {y}");
} */

/* fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}"); // error
    println!("{s2}");
} */

/* fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");
} */

/* fn main() {
    let s = String::from("hello");
    // takes_ownership(s);
    takes_ownership(s.clone());
    // println!("{s}");

    let x = 6;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} */

/* fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // println!("{}", s2); // error
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} */

/* fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // println!("{}", s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
} */

// 不可变借用，只是借用，并没有起所有权
/* fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize{
   s.len()
} */

// 无法改变 不可变借用 的值
/* fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(s: &String) {
    s.push_str(", world!");
} */

// 可变借用
/* fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String)   {
    s.push_str(", world!")
} */

// slice
/* fn main() {
    let s = String::from("hello world");
    println!("{}", first_word(&s))
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
} */

fn main() {
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    println!("{}", word);

    s.clear();
    // println!("{}", word);
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
