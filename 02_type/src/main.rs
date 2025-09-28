fn main() {
    let guess: u32 = "52".parse().expect("Not a number");
    println!("guess is: {}", guess);

    // 标量 scalar 是不可再分的值
    // 1.整数类型 
    // 有符号整数：i8, i16, i32, i64, i128, isize

    // 无符号整数：u8, u16, u32, u64, u128, usize

    //每个有符号类型规定的数字范围是 -(2n - 1) ~ 2n - 1 - 1，其中 n 是该定义形式的位长度。所以 i8 可存储数字范围是 -(27) ~ 27 - 1，即 -128 ~ 127。无符号类型可以存储的数字范围是 0 ~ 2n - 1，所以 u8 能够存储的数字为 0 ~ 28 - 1，即 0 ~ 255。此外，isize 和 usize 类型取决于程序运行的目标平台：64 位平台中它们是 64 位的，32 位平台中它们是 32 位的。

    // 整数的溢出
    // 比如u8的话，如果数字是256，就理论上会溢出。但是如果不是生产模式是debug模式的话，会报错。如果是生产模式的话，会进行模运算，即 256 % 256 = 0
    // rust之中使用panic来表示报错
    // 在使用--release的时候，rust不检测会导致panic的整数溢出。会使用一个溢出包裹的行为，就是256会成为0，257会成为1。不要依赖和使用这样的溢出包裹的行为。



    // 2.浮点数类型
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // 数字运算
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // 结果为0
    let _remainder = 43 % 5;

    // 3.布尔类型

    // 4.字符类型
    // char类型
    // 表示的是一个Unicode的标量值，所以可以表示中文、日文、韩文、emoji、ASCII等
    let c = 'z';
    let _heart_eyed_cat = '😻';
    let _z = 'ℤ';
    println!("value of c is: {}", c);
    println!("value of heart_eyed_cat is: {}", _heart_eyed_cat);
    println!("value of z is: {}", _z);

    // 复合类型
    // 元组：元组是一个可以有多类型组成的一个类型。rust有两种基本的符合类型：元组和数组。
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = _tup;
    println!("value of x is: {}", x);
    println!("value of y is: {}", y);
    println!("value of z is: {}", z);
    println!("value of tup is: {:#?}", _tup);
    println!("value of tup is: {:#?}", _tup);

    // 数组
    // 数组是一个只能有相同类型组成的一个类型。具有固定的长度。
    // 你希望把数组放到栈，而不是堆，那么你就需要使用数组。
    // vector是向量，他是动态数组，非常灵活。数组没有那么灵活
    let _arr: [i32; 3] = [1, 2, 3];
    let _arr = [3; 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("value of arr is: {:?}", _arr);
    println!("value of arr is: {:?}", _arr);
    println!("value of a is: {:?}", _a);
    let first = _a[0];
    let second = _a[1];
    println!("value of first is: {}", first);
    println!("value of second is: {}", second);

    let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
    println!("value of a is: {:?}", a);
    
    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
