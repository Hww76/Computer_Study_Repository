
/// 标量学习
fn main(){
    // 使用 let 语句进行变量绑定
    // 你可以不知道什么是变量绑定，只要抄下来就对了

    // 在变量绑定的时候可以使用 ':' 标注类型
    let logical: bool = true;
    println!("1 == 1 is {}", logical);

    // 两种不同的标注方法
    let a_float: f32 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // 如果不标注则会使用默认的类型（由字面量决定） 
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 整数后面代表储存用的 bit 数
    // 这决定了它的范围
    let int8: i8 = 127; // [-128,127] 8位有符号整型
    let uint8: u8 = 255; // [0, 255] 8位无符号整型
    let int16: i16 = 32767; // [-32768, 32767] 16位有符号整型
    let uint16: u16 = 65535; // [0, 65535] 16位无符号整型

    // error!! 下面的四行代码无法运行
    // 你可以注释掉以下的四行代码
    // 请注意编译器的错误提示，isize 与 usize 会随着机器变化

    // let pointer: isize = 2333333333333333333333; // pointer size
    // let int64: i64 = 2333333333333333333333;    // i64 与 64位机上的 isize 相同
    // let upointer: usize = 6666666666666666666666; // pointer size
    // let upointer: u64 = 6666666666666666666666; // u64 与 64位机上的 usize 相同

    // 编译器可以根据上下文推断类型
    // let mut 定义的是可以变的变量，你还不需要知道它的意思
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // error: 下面这段代码有错误！
    // 你不能这么做不是因为它计算的结果不合法
    // 而是它们类型不相同，并不能进行计算（虽然看上去很明显可以）
    // println!("1u32 - 1i32 = {}", 1u32 - 1i32);
    println!("2u32 - 1u32 = {}", 2u32 - 1u32);

    // 字符类型
    let en: char = 'x'; // char: 4B
    let zh: char = '中'; // unicode 四个字节可以表示现在的所有字符

    return (); // unit type 是所有没有标注返回值的函数的默认返回类型
}