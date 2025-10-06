fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 这里传递的是s1的引用，它允许你使用这个值而不传递其所有权。

    println!("The length of '{}' is {}.", s1, len);

    // &的相反的使用方法是解引用*。dereferencing

    // 如果我们尝试修改变量呢？会如何
    // let s = String::from("hello");

    // change(&s); // 这里会报错，因为s是不可变的

    // 那么我们该如何修改这个变量呢？
    let mut s = String::from("hello");
    change(&mut s); // 这里我们传递了一个可变引用给change函数
    println!("s is: {}", s);
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 这里会报错，因为s是不可变的
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
