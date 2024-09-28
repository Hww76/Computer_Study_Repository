#![no_std] // 来告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core（core库不需要操作系统的支持）。
#![no_main] //告诉编译器我们没有一般意义上的 main 函数，并将原来的 main 函数删除。
mod lang_items;

// fn main() {
//     // println!("hello world");
// }


/*
尝试编译，报错无法继续，说是和目标平台为riscv64gc-unknown-none-elf差别不大，但是不是改了目标平台将能马上生成目标文件的
(base) somls@ubt:x86_os$ cargo build
warning: `/home/somls/Desktop/workspace/studyspace/os_learn/rCore/chapter1/x86_os/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
warning: `/home/somls/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
warning: `/home/somls/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
   Compiling x86_os v0.1.0 (/home/somls/Desktop/workspace/studyspace/os_learn/rCore/chapter1/x86_os)
error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

error: could not compile `x86_os` (bin "x86_os") due to 1 previous error
*/