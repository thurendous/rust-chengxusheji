fn main() {
    fist_fn();

    println!("=================fist_fn end==================");

    let s = String::from("hello11"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                     // 这里s的所有权被转移到了函数takes_ownership中,所以s就不会再有效了
    // println!("s is: {}", s); // 这里会报错，因为s的所有权已经被转移到了函数takes_ownership中
                                     
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("x is: {}", x);

    let s1 = String::from("hello");
    let _s2 = s1;
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // here, some_integer goes out of scope. Nothing special happens.


fn fist_fn() {
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


