fn main() {
    let s  = "hello";

    let s2 = s;

    println!("s2 is: {}", s2);

    let s3 = String::from("hello");
    let s4 = s3; // 这里的s3并没有复制这个堆上的string的数据。

    // println!("s3 is: {}", s3); // 这里会报错，因为s3的所有权已经转移给了s4
    println!("s4 is: {}", s4);

    let mut s = String::from("hello");
    s.push_str(", world"); // 这里为何string
    println!("s is: {}", s);

    // 深度复制
    let str = String::from("hello");
    let str2 = str.clone();
    println!("str is: {}", str);
    println!("str2 is: {}", str2);
}
