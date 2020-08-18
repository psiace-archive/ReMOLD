fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("The value of x and y is {:?} and {:?}", x, y);

    // 加法
    let sum = 5 + 10;

    println!("The value of sum is {}", sum);

    // 减法
    let difference = 95.5 - 4.3;

    println!("The value of difference is {}", difference);

    // 乘法
    let product = 4 * 30;

    println!("The value of product is {}", product);

    // 除法
    let quotient = 56.7 / 32.2;

    println!("The value of quotient is {}", quotient);

    // 取余
    let remainder = 43 % 5;

    println!("The value of remainder is {}", remainder);

    let t = true;

    let f: bool = false; // 显式指定类型注解

    println!("The value of t and f is {} and {}", t, f);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // 元组，带类型注解

    let (x, y, z) = tup; // 模式匹配解构

    println!("The value of y is: {}", y);

    let five_hundred = tup.0; // . 号加值索引访问

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is {}", five_hundred);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {}", first);

    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element); Thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:62:19
}
