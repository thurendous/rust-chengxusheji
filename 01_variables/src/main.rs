fn main() {
    let x = 5; // immutable variable
    let mut _y = 6; // mutable variable
    println!("1st value of y is: {}", _y);
    _y = 7;
    println!("2nd value of y is: {}", _y);
    println!("value of x is: {}", x);

    // constant variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    
    // shadow
    let z = 8;
    println!("value of z firstly is: {}", z);
    let z = z + 1;

    {
        let z = z * 2;
        println!("value of z in inner scope is: {}", z);
    }

    println!("value of z is: {}", z);

    let spaces = "   ";
    // 使用let可以改变这个变量的类型，因为使用了let，所以之前的变量被遮蔽了
    let spaces = spaces.len();
    println!("value of spaces is: {}", spaces);

    // 如果不使用shadow的话，那么需要使用mut来声明一个可变变量，这样的话会报错

    
}
