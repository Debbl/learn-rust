/* #[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("aaa"),
        email: String::from("bbb"),
        sign_in_count: 12442,
    };
    user.username = String::from("debbl");
    let mut user1 = build_user(String::from("ccc"), String::from("j"));
    let user2 = User {
        email: String::from("me@aiwan.run"),
        ..user1
    };
    // println!("{:#?}", user1);
    user1.username = String::from("1111");
    println!("{:#?}", user);
    println!("Hello, world!");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
 */
/*
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?}, {:#?}.", black, origin);
} */

// 类单元结构体 unit-like-structs
/* struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
} */

/* #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法 .
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    /*     let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    ); */

    /*     let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels", area(rect1)); */

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle::new(1, 1);
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect2)
    );
    dbg!(&rect2);
    println!(
        "The area of the rectangle is {} square pixels",
        rect2.get_area()
    );
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

/* fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
} */

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
} */

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(30);
    println!("{}", rect1.get_area());
}
