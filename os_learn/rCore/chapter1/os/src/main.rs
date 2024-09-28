#![no_std] // 来告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core（core库不需要操作系统的支持）。
#![no_main] //告诉编译器我们没有一般意义上的 main 函数，并将原来的 main 函数删除。
mod lang_items;

// fn main() {
//     // println!("hello world");
// }
