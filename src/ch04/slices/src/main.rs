// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。
fn main() {
    let mut s = String::from("hello world");

    // let word = first_word(&s); // word 的值为 5
    // s.clear(); // 这里 清空了字符串，使其等于 ""

    let word = first_word2(&s);
    // s.clear(); // 错误！

    println!("the first word is: {}", word);

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    // 字符串 slice
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // 对于 Rust 的 .. range 语法，如果想要从索引 0 开始，可以不写两个点号之前的值。换句话说，如下两个语句是相同的：
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &[..2];

    // 依此类推，如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。这意味着如下也是相同的：
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // 也可以同时舍弃这两个值来获取整个字符串的 slice。所以如下亦是相同的：
    let slice = &s[0..len];
    let lice = &s[..];

    // 字符串字面值就是 slice
    let s = "Hello, world!";
    // 这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

    // 字符串 slice 作为参数
    let my_string = String::from("hello world");

    // `first_word3` 适用于 `String`（的 slice），部分或全部
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word3` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";
    // `first_word3` 适用于字符串字面值，部分或全部
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word3(my_string_literal);

    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// 示例 4-7：first_word 函数返回 String 参数的一个字节索引值
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 重写 first_word 来返回一个 slice。“字符串 slice” 的类型声明写作 &str：
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 示例 4-9: 通过将 s 参数的类型改为字符串 slice 来改进 first_word 函数
fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
