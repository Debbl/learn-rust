/**
 * ## 变量 （variables）
 * - 变量声明使用 let 关键字
 * - 默认情况下，变量是不可变的（Immutable）
 * - 声明变量是，在变量前面加上 mut ，就可以使变量可变
 *
 * ## 常量 （constant）
 * - 绑定值后不可变
 * - 不可以使用 mut 关键字，类型必须被标注
 * - 常量可以在任何作用域进行声明，包括全局作用域
 * - 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行是才能计算出的值
 * > 在程序运行期间，常量在其声明的作用域内一直有效
 * > 命名规范 全大写 单词用下划线分开
 *
 * ## Shadowing 隐藏
 * - 使用相同的名字声明新的变量，新的变量会 shadow 之前声明的同名变量
 * - 后续代码使用的是新的变量
 * - 变量有 mnt 不可以 shadow
 *
 * ## 标量类型
 * ### 整数类型
 * #### 无符号
 * - u8 u16 u32 u128 usize
 * #### 有符号
 * - i8 i16 i32 i128 isize
 * > size 由计算机的架构决定 arch
 *
 * ### 整数的字面值
 * - Decimal 98_222
 * - Hex 0xff
 * - Octal 0o77
 * - Binary ob000_111
 * - Byte (u8 only) b'A'
 *
 * ### 浮点类型
 * - f32 f64(默认)
 *
 * ### 数值操作
 *
 * ### 布尔类型
 * bool
 * - true false
 *
 * ### 字符类型
 * char 4byte
 *
 * ## 复合类型
 * ### Tuple
 * - 长度固定
 * ### Array 数组
 * - 每个元素的类型必须相同
 */

// 变量 常量
// const MAX_POINTS: u32 = 100_000;

// fn main() {
//     const MAX_POINTS: u32 = 100_000;
//     println!("Hello, world!");

//     let mut x = 5;
//     println!("The value of x is {}", x);

//     x = 6;
//     println!("The value of x is {}", x);
// }

// 隐藏
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is {}", x);
// }

// fn main() {
//     let spaces = "    ";
//     let spaces = spaces.len();
//     println!("The value is {}", spaces);
// }

// 标量类型
// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number");

//     println!("{}", guess);
// }

// 复合类型
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // 解构
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    // 访问
    println!("{}", tup.0);
    // 数组 [类型;长度]
    let arr = [1, 2, 3, 4, 5, 6];
    let a = [3; 5];
    println!("{}, {}", arr[0], a[0]);
}
