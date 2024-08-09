fn main() {
    {
        // s 在这里无效，它尚未声明
        let s = "hello"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s 不再有效

    // Rust 有另一种字符串类型，String。这个类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本。
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    // 内存与分配
    // Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，
      // s 不再有效
      // 这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。
      // 当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。
      // Rust 在结尾的 } 处自动调用 drop。

    // 为了确保内存安全，在 let s2 = s1; 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。
    // 看看在 s2 被创建之后尝试使用 s1 会发生什么；这段代码不能运行：
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
    */

    // 如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // 所有权与函数
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

// 返回值与作用域 示例 4-4: 转移返回值的所有权
fn main2() {
    let s1 = gives_ownership(); // gives_ownership 将返回值转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中，它也将返回值移给 s3
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移动走，所以什么也不会发生。s1 离开作用域并被丢弃。

fn gives_ownership() -> String {
    // gives_ownership 会将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_sring  并移出给调用的函数
}

// 示例 4-5: 返回参数的所有权
fn main4_5() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
