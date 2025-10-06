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
    // 因为我们传递了一个可变引用，所以我们可以修改这个变量
    // 但是可变引用有一个很大的限制，他就是同一时间只能有一个可变引用
    // 这个限制可以防止数据竞争
    // 数据竞争会导致未定义的行为
    // 数据竞争的定义是：
    // 1. 两个或更多的指针同时访问同一数据
    // 2. 至少有一个指针被写入
    // 3. 没有同步数据访问的机制
    println!("s is: {}", s);

    let mut s1 = String::from("hello s3 borrow once");
    let s3 = &mut s1;
    // let s4 = &mut s1; // 这里会报错，因为同一时间只能有一个可变引用
    println!("s3 is: {}", s3);

    let mut _s4 = &mut s1;  // ✅ 允许，因为 s3 的可变借用在上面那一行就结束了
    // println!("{}", s3); // ❌ 如果把这行解注释，就会报错：
    //                     // 因为这会把 s3 的“最后一次使用”推迟到这里，
    //                     // 导致和 _s4 的可变借用重叠。
    // 要点拆解：
	// 1.	可变借用只能同时存在一个：在 s3 活跃时，不能再拿 &mut s1。（NLL就是词法生命周期，是2018年对rust引入的概念，它允许了这个机制的产生）
	// 2.	NLL 让借用在“最后一次使用”后立即结束：编译器看出 s3 在 println! 之后不再被用到，于是把 s3 的借用期截止在那一行末尾。这样，下一行再借用 &mut s1（赋给 _s4）就不冲突。
	// 3.	不是调用了 drop：引用类型本身没有需要显式释放的资源；这里结束的是借用关系，不是运行时的析构。即使你不调用 drop(s3)，编译器也会因为 NLL 把借用提前结束。
	// 4.	println! 里发生了什么：宏会把 s3 以只读方式使用（&mut String 会自动协变/解引用为 &String / &str），用完这次读取后，s3 的可变借用就不再被使用，于是借用到此为止。

    // 大括号可以让生命两个可变引用
    let mut s = String::from("hello s2 borrow twice");

    {
        let r1 = &mut s;
        println!("r1 is: {}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 is: {}", r2);

    let mut s = String::from("hello");

    // 这里会发生什么呢？
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题，因为这里都是只读引用
    // let r3 = &mut s; // 大问题，因为这里同时存在一个不可变引用和一个可变引用。他们也不能同时存在的

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);

    // 所以我们可以这样写，这里注意r1和r2的声明位置和使用位置。如果r1、r2被使用了，那么他的作用域（生命周期）就结束了，r3就可以被声明了
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    let reference_to_nothing = dangle(); // 这里会发生什么呢？
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

fn dangle() -> &String { // 这里会发生什么呢？
    let s = String::from("hello");
    &s
    // 这里会报错，因为s是局部变量，它离开作用域后，其所有权被转移给了&s。但是&s是局部变量，它离开作用域后，其所有权被转移给了dangle函数。但是dangle函数返回的是&s，所以会报错
}
