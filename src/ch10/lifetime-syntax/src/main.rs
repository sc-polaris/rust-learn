/*
    // 生命周期避免了悬垂引用
    // 示例 10-16：尝试使用离开作用域的值的引用
    fn main() {
        let r;
        {
            let x = 5;
            r = &x;
        }

        println!("r: {r}");
    }

    // 借用检查器
    // 示例 10-17：r 和 x 的生命周期注解，分别叫做 'a 和 'b
    fn main() {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {r}");   //          |
    }                         // ---------+

    // 示例 10-18：一个有效的引用，因为数据比引用有着更长的生命周期
    fn main() {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {r}");   //   |       |
                              // --+       |
    }                         // ----------+

    // 函数中的泛型生命周期
    // 示例 10-19：main 函数调用 longest 函数来寻找两个字符串 slice 中较长的一个
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }
    // 示例 10-20：一个 longest 函数的实现，它返回两个字符串 slice 中较长者，现在还不能编译
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // 会出现有关生命周期的错误
    // 提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！

    // 生命周期注解语法
    &i32        // 引用
    &'a i32     // 带有显式生命周期的引用
    &'a mut i32 // 带有显式生命周期的可变引用

    // 我们希望函数签名表达如下限制：也就是这两个参数和返回的引用存活的一样久。（两个）参数和返回的引用的生命周期是相关的。就像示例 10-21 中在每个引用中都加上了 'a 那样。
    // 示例 10-21：longest 函数定义指定了签名中所有的引用必须有相同的生命周期 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 示例 10-22：通过拥有不同的具体生命周期的 String 值调用 longest 函数
    fn main() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }
    }

    // 示例 10-23：尝试在 string2 离开作用域之后使用 result
    fn main() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {result}");
    }

    // 结构体定义中的生命周期注解
    // 示例 10-24：一个存放引用的结构体，所以其定义需要生命周期注解
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // 生命周期省略（Lifetime Elision）
    // 示例 10-25：示例 4-9 定义了一个没有使用生命周期注解的函数，即便其参数和返回值都是引用
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    // 这个函数没有生命周期注解却能编译是由于一些历史原因：在早期版本（pre-1.0）的 Rust 中，这的确是不能编译的。每一个引用都必须有明确的生命周期。那时的函数签名将会写成这样：
    fn first_word<'a>(s: &'a str) -> &'a str {
    // 在编写了很多 Rust 代码后，Rust 团队发现在特定情况下 Rust 程序员们总是重复地编写一模一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。接着 Rust 团队就把
    // 这些模式编码进了 Rust 编译器中，如此借用检查器在这些情况下就能推断出生命周期而不再强制程序员显式的增加注解。
    // 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。

    // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。

       1. 第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数就有两个不同的生命周期参数，
          fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
       2. 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
       3. 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出生命周期参数被赋予
          self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。

    // 方法定义中的生命周期注解
    // impl 之后和类型名称之后的生命周期参数是必要的，不过因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    // 这里是一个适用于第三条生命周期省略规则的例子：
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    // 静态生命周期
    // 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
    let s: &'static str = "I have a static lifetime.";

*/
use std::fmt::Display;

fn main() {}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 指定生命周期参数的正确方式依赖函数实现的具体功能。例如，如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。如下代码将能够编译：
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值。然而它将会是一个悬垂引用，
// 因为它将会在函数结束时离开作用域。尝试考虑这个并不能编译的 longest 函数实现：
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
// 即便我们为返回值指定了生命周期参数 'a，这个实现却编译失败了，因为返回值的生命周期与参数完全没有关联。

// 结合泛型类型参数、trait bounds 和生命周期
// 这个是示例 10-21 中那个返回两个字符串 slice 中较长者的 longest 函数，不过带有一个额外的参数 ann。ann 的类型是泛型 T，它可以被放入任何实现了 where 从句中指定的 Display trait 的类型。
// 这个额外的参数会使用 {} 打印，这也就是为什么 Display trait bound 是必须的。因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
