fn main() {
    println!("Hello, world!");

    another_function();
    another_function_arg(5);
    another_function_mul_args(5, 6);

    // let x = (let y = 6); 不能把 let 语句赋值给另一个变量

    let y = {
        // 函数体
        let x = 3;
        x + 1 // 不加分号，返回 x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let z = plus_one(5);

    println!("The value of z is: {}", z);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_arg(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_mul_args(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // ; 加上分号后，返回的类型变成空元组，导致类型不匹配
}
