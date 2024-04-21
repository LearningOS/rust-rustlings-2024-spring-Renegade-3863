// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    // rust的巨大优势, 可以用{}占位符代替C/C++中的%d, %s等用于说明不同类型的输出格式占位符
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
