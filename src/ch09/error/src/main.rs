// fn main() {
//     panic!("crash and burn");
// }

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{fs, io};

// fn main() {
// let v = vec![1, 2, 3];
//
// v[99];

// 示例 9-3：打开文件
// let greeting_file_result = File::open("hello.txt");

// 示例 9-4：使用 match 表达式处理可能会返回的 Result 成员
// let greeting_file = match greeting_file_result {
//     Ok(file) => file,
//     Err(error) => panic!("Problem opening the file: {error:?}"),
// };

// 示例 9-5：使用不同的方式处理不同类型的错误
// let greeting_file = match greeting_file_result {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {e:?}"),
//         },
//         other_error => {
//             panic!("Problem opening the file: {other_error:?}");
//         }
//     },
// };
// 这是另一个编写与示例 9-5 逻辑相同但是使用闭包和 unwrap_or_else 方法的例子：
// let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//     if error.kind() == ErrorKind::NotFound {
//         File::create("hello.txt").unwrap_or_else(|error| {
//             panic!("Problem creating the file: {:?}", error);
//         })
//     } else {
//         panic!("Problem opening the file: {:?}", error);
//     }
// });

// 失败时 panic 的简写：unwrap 和 expect
// 如果调用这段代码时不存在 hello.txt 文件，我们将会看到一个 unwrap 调用 panic! 时提供的错误信息：
// let greeting_file = File::open("hello.txt").unwrap();

// 还有另一个类似于 unwrap 的方法它还允许我们选择 panic! 的错误信息：expect。使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。expect 的语法看起来像这样：
// let greeting_file =
//     File::open("hello.txt").expect("hello.txt should be included in this project");
// expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，而不像 unwrap 那样使用默认的 panic! 信息。
// 在生产级别的代码中，大部分 Rustaceans 选择 expect 而不是 unwrap 并提供更多关于为何操作期望是一直成功的上下文。如此如果该假设真的被证明是错的，你也有更多的信息来用于调试。
// }

// 传播错误
// 例如，示例 9-6 展示了一个从文件中读取用户名的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// 示例 9-7：一个使用 ? 运算符向调用者返回错误的函数
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// 示例 9-8：问号运算符之后的链式方法调用
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// 示例 9-9: 使用 fs::read_to_string 而不是打开后读取文件
/*
   将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
   当然，这样做就没有展示所有这些错误处理的机会了，所以我们最初就选择了艰苦的道路。
*/
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// 示例 9-10: 尝试在返回 () 的 main 函数中使用 ? 的代码不能编译
// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }

// 示例 9-11: 在 Option<T> 值上使用 ? 运算符
/*
    这个函数返回 Option<char> 因为它可能会在这个位置找到一个字符，也可能没有字符。这段代码获取 text 字符串 slice 作为参数并调用其 lines 方法，这会返回一个字符串中每一行的迭代器。
    因为函数希望检查第一行，所以调用了迭代器 next 来获取迭代器中第一个值。如果 text 是空字符串，next 调用会返回 None，此时我们可以使用 ? 来停止并从 last_char_of_first_line 返回
    None。如果 text 不是空字符串，next 会返回一个包含 text 中第一行的字符串 slice 的 Some 值。

    ? 会提取这个字符串 slice，然后可以在字符串 slice 上调用 chars 来获取字符的迭代器。我们感兴趣的是第一行的最后一个字符，所以可以调用 last 来返回迭代器的最后一项。这是一个 Option，
    因为有可能第一行是一个空字符串，例如 text 以一个空行开头而后面的行有文本，像是 "\nhi"。不过，如果第一行有最后一个字符，它会返回在一个 Some 成员中。? 运算符作用于其中给了我们一个简
    洁的表达这种逻辑的方式。如果我们不能在 Option 上使用 ? 运算符，则不得不使用更多的方法调用或者 match 表达式来实现这些逻辑。
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 示例 9-12: 修改 main 返回 Result<(), E> 允许对 Result 值使用 ? 运算符
/*
    Box<dyn Error> 类型是一个 trait 对象（trait object）第十七章 顾及不同类型值的 trait 对象” 部分会做介绍。目前可以将 Box<dyn Error> 理解为 “任何类型的错误”。在返回 Box<dyn Error>
    错误类型 main 函数中对 Result 使用 ? 是允许的，因为它允许任何 Err 值提前返回。即便 main 函数体从来只会返回 std::io::Error 错误类型，通过指定 Box<dyn Error>，这个签名也仍是正确的，甚
    至当 main 函数体中增加更多返回其他错误类型的代码时也是如此。

    当 main 函数返回 Result<(), E>，如果 main 返回 Ok(()) 可执行程序会以 0 值退出，而如果 main 返回 Err 值则会以非零值退出；成功退出的程序会返回整数 0，运行错误的程序会返回非 0 的整数。Rust
    也会从二进制程序中返回与这个惯例相兼容的整数。

    main 函数也可以返回任何实现了 std::process::Termination trait 的类型，它包含了一个返回 ExitCode 的 report 函数。
*/
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

mod test {
    use std::net::IpAddr;

    #[test]
    fn ip() {
        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP address should be valid");
    }
}
