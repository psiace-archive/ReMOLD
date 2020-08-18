fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { // 类型不匹配，一定需要 bool
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six" 类型不匹配，if 的每个分支的可能的返回值都必须是相同类型
    };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop { //  一遍又一遍地执行一段代码直到你明确要求停止
        counter += 1;

        if counter == 10 {
            break counter * 2; // break 跳出，并返回值
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() { // 使用 for..in 来循环
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // rev，用来反转 range
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
