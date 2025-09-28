fn main() {
    println!("Hello, world! from fn_demo");

    another_function();

    another_function_with_x(5);
    another_function_with_x_and_y(5, 6);

    // rust中，函数又一系列的语句组成。也可以选择以表达式结尾。
    // 目前为止，介绍的函数还没有用表达式结尾的
    // Rust是一个以基于表达式的语言
    // 以下是一个语句
    let _x = 6; // 语句不会返回值
    // 比如Rust中不可以这么写：let _y = (let x = 6);语句时没有返回值的
    // 以下是一个表达式
    let _y = _x + 1; // 表达式会返回值

    let expression = expression();
    println!("The value of expression is: {}", expression);

    five();
    println!("The value of five is: {}", five());

    empty_returned_value(5);
    println!("The value of empty_returned_value is: {:?}", empty_returned_value(5));
}


fn another_function() {
    println!("Another function.");
}

fn another_function_with_x(x: i32) {
    println!("Another function with x: {}", x);
}

fn another_function_with_x_and_y(x: i32, y: i32) {
    println!("Another function with x: {} and y: {}", x, y);
}

fn expression() -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    y // 表达式的结尾时没有分号的
}

fn five() ->u32 {
    5
}

fn empty_returned_value(_x: i32) -> () {
    println!("The value of emptyReturnedValue is: {:?}", _x);
}

