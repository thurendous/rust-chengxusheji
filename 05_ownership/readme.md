## Stack and Heap

很多语言中，你并不需要注意堆和栈。一个是 heap，一个是 stack。

而在 Rust 之中，值位于堆还是栈上，很大程度会影响语言的行为以及为何必须做出这样的抉择。

堆和栈都是运行时候提供的内存。但是结构不同。

-   栈（stack）：以放入东西的顺序存储值，并且以相反的顺序拿出去。这也被称作：last in, first out。栈上只可以存储固定大小的东西。大小未知或者大小会变化的数据要改为存储在堆上。
-   堆（heap）：堆是缺乏组织的。当向堆中存入数据的时候。

当你学会了 rust 的所有权，你就不需要记住堆和栈的这些具体内容了。你只需要知道所有权是为了控制 heap 上的数据，让他更加高效的存储以及被利用而存在的。

## 所有权规则

首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

-   Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
-   值在任一时刻有且只有一个所有者。
-   当所有者（变量）离开作用域，这个值将被丢弃。

如果是{}作用域存在的话，这个里边声明的东西只能在这个里边有效。

```
{
    let s = "hello";
    // 使用s
}
```

字符串字面量的话，编译的时候我们就知道其内容了。所以文本被肢解应变码进入了最终的可执行文件之中。这使得字符串字面量快速且高效。不过这些特性只得益于字符串字面量的不可变性质。

```rust
fn main() {
    let s = "hello world!"; // 字符串字面量变量。string literal。他的内容在编译时候已经确认了。直接存到了程序的二进制文件之中。在程序的数据段.rodata
    // 程序运行的时候，s只是只想这段制度内存的引用。也就是说，这个字符串是不可变的。
    // 不能修改它；他的大小和地址都是固定的。类似solidity的constant。所有尝试修改它的话都会报错。
    println!("{}", s);
}
```

String 类型（堆分配，可变）

当我们需要在程序运行时动态生成、修改或接收输入字符串时，就不能使用字面量了。

示例

```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world"); // ✅ 可以修改
    println!("{}", s);
}
```

分析
• String 类型的数据结构在 栈（stack） 上存储指针、长度和容量；
• 实际的字符内容 "hello" 存储在 堆（heap） 上；
• 当我们调用 .push_str() 时：
• Rust 可能在堆上重新分配更多内存；
• 然后将新的字符串内容写进去；
• 这个行为在运行时决定。

rust 没有 GC 的策略。所以需要一种垃圾回收的机制。它采用的是一个不同的策略。内存在拥有他的变量离开作用域后就被自动释放。

```rust
    {
     let s = String::from("hello"); // 从此处起，s 开始有效

        // 使用 s
    }                                  // 此作用域已结束，
                                       // s 不再有效
```

当变量离开了作用域的时候，rust 会帮助我们的调用一个特殊的函数叫做`drop`。这里的`String`的作者可以防止释放内存的代码。Rust 在最后的括号的地方自动调用`drop`函数。

```rust
    let s1 = String::from("hello");
    let s2 = s1; // 这里的s1并没有复制这个堆上的string的数据。
    // println!("{}", s1);
```

而这里如果你要打印这个 s3 就会报错。

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

这个`let s4 = s3;`可能就是别的语言的“浅拷贝”。其实在 rust 之中这个操作被叫做移动(move)。而不是“浅拷贝”。这个操作可以背理解为 s1 被移动到了 s2.

这样就清楚了，s1 在移动以后就消失了，那么只有 s2 有效了。当它离开了自己的作用域，他就释放了自己的内存了，完毕。

另外这里还隐藏了一个设计选择。rust 永远不会主动去做“深拷贝”。因此，任何自动的复制可以被认为对运行时的性能影响比较小。（因为深拷贝会在 heap 之中从新创建一个新数据，而比如这里的 String 可能会很大）

### 克隆

如果我们确实需要深度复制一个 String 上的数据，而不是仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。

这个 clone 的用法如下：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

这段代码就可以正常运行了。因为明确深度复制了这个 s1.那么就会是在 stack 和 heap 上都有这个数据存在了。stack 上是指针、长度、容量。

### 只在栈 stack 上的数据的拷贝

```rust
let x = 5;
let y = x;
println!("{}", )
```

这里的 primitive 的数据在拷贝的时候它整体都是在 stack 上的，所以拷贝他的速度是非常快的。这里也没有深拷贝的这样的说法。

Rust有一个叫做Copy的`trait`的特殊的标注，可以用在类似整型这样的存储在栈上的类型上。

如果一个类型实现了Copy的trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。

Rust不允许自身或者任何部分实现了`Drop`的trait的类型使用Copy的标注。

如果我们对其值离开作用域时需要特殊处理的类型使用Copy标注，将会出现一个编译错误。

要学习如何为你的类型添加 Copy 标注以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。

那么哪些类型实现了 Copy trait 呢？你可以查看给定类型的文档来确认，不过作为一个通用的规则，任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。如下是一些 Copy 的类型：

- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

返回值与作用域
返回值也可以转移所有权。示例 4-4 与示例 4-3 一样带有类似的注释。

文件名: src/main.rs

```rust
fn main() {
  let s1 = gives_ownership();         // gives_ownership 将返回值
                                      // 移给 s1

  let s2 = String::from("hello");     // s2 进入作用域

  let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                      // takes_and_gives_back 中,
                                      // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
                                           // 调用它的函数

  let some_string = String::from("yours"); // some_string 进入作用域

  some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

  a_string  // 返回 a_string 并移出给调用的函数
}
```

示例 4-4: 转移返回值的所有权

变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。

我们可以使用元组来返回多个值，如示例 4-5 所示。

文件名: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```

示例 4-5: 返回参数的所有权

但是这未免有些形式主义，而且这种场景应该很常见。幸运的是，Rust 对此提供了一个功能，叫做 引用（references）。
