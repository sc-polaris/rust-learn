fn main() {
    println!("Hello, world!");
}

// 示例 4-5: 引用 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 与指针不同，引用确保指向某个特定类型的有效值。
fn main4_5_reference() {
    let s1 = String::from("hello");

    let len = calculate_length2(&s1); // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。

    println!("The length of '{s1} is {len}.");
}

fn calculate_length2(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// 示例 4-6：尝试修改借用的值
// 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
/*
   fn main4_6() {
       let s = String::from("hello");

       change(&s);
   }

   fn change(some_string: &String) {
       some_string.push_str(", world");
   }
*/

// 我们通过一个小调整就能修复示例 4-6 代码中的错误，允许我们修改一个借用的值，这就是 可变引用（mutable reference）：
fn main4_6() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。这些尝试创建两个 s 的可变引用的代码会失败：
/*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
*/

// 让我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免：
/*
    fn main_dangling_reference() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {    // dangle 返回一个字符串的引用
        let s = String::from("hello");  // s 是一个新字符串

        &s  // 返回字符串 s 的引用
    }   // 这里 s 离开作用域并被丢弃。其内存被释放。

    因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许我们这么做。
*/

// 这里的解决方法是直接返回 String：
fn main_dangling_reference() {
    let reference_to_nothing = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

/*
    引用的规则
    让我们概括一下之前对引用的讨论：
    1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    2. 引用必须总是有效的。
*/
