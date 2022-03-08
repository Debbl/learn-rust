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

fn main() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value is {}", spaces);
}
