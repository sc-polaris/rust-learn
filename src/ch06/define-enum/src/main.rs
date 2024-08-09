enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {}

// 我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
// IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值：
enum IpAddr2 {
    V4(String),
    V6(String),
}

// 用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。
// 如果我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String，这就不能使用结构体了。枚举则可以轻易的处理这个情况：
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // 没有关联任何数据。
    Move { x: i32, y: i32 },    // 类似结构体包含命名字段。
    Write(String),              // 包含单独一个 String。
    ChangeColor(i32, i32, i32), //  包含三个 i32。
}

// 定义一个如 Message 中所示那样的有关联值的枚举的方式和定义多个不同类型的结构体的方式很相像，
// 除了枚举不使用 struct 关键字以及其所有成员都被组合在一起位于 Message 类型下。如下这些结构体可以包含与之前枚举成员中相同的数据：
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

// 不过，如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像 Message 枚举那样，轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 枚举
    // Option 是标准库定义的另一个枚举。Option 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。
    /*
       enum Option<T> {
           None,
           Some(T),
       }
    */
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    /*
       那么，Option<T> 为什么就比空值要好呢？
       简而言之，因为 Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用
       Option<T>。例如，这段代码不能编译，因为它尝试将 Option<i8> 与 i8 相加：
    */
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}
