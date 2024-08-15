use std::fmt::{Debug, Display};

// 示例 10-12：Summary trait 定义，它包含由 summarize 方法提供的行为
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类型实现 trait

// 示例 10-13：在 NewsArticle 和 Tweet 类型上实现 Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 示例 10-14：Summary trait 的定义，带有一个 summarize 方法的默认实现
// pub trait Summary2 {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// impl Summary2 for NewsArticle {}

/*
    默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。如此，trait 可以提供很多有用的功能而只需要实现指定一小部分内容。例如，
    我们可以定义 Summary trait，使其具有一个需要实现的 summarize_author 方法，然后定义一个 summarize 方法，此方法的默认实现调用 summarize_author 方法：
    注意无法从相同方法的重载实现中调用默认方法。
*/
pub trait Summary2 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary2 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 作为参数
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait Bound 语法
// impl Trait 语法适用于直观的例子，它实际上是一种较长形式我们称为 trait bound语法的语法糖。它看起来像：
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// impl Trait 很方便，适用于短小的例子。更长的 trait bound 则适用于更复杂的场景。例如，可以获取两个实现了 Summary 的参数。使用 impl Trait 的语法看起来像这样：
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// 这适用于 item1 和 item2 允许是不同类型的情况（只要它们都实现了 Summary）。不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能：
// 泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// 通过 + 指定多个 trait bound
// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
// pub fn notify(item: &(impl Summary + Display)) {}

// + 语法也适用于泛型的 trait bound：
// pub fn notify<T: Summary + Display>(item: &T) {}

// 通过指定这两个 trait bound，notify 的函数体可以调用 summarize 并使用 {} 来格式化 item。

// 通过 where 简化 trait bound
// 然而，使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。
// 为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。所以除了这么写：
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// 还可以像这样使用 where 从句：
/*
   fn some_function<T, U>(t: &T, u: &U) -> i32
   where
       T: Display + Clone,
       U: Clone + Debug,
   {}
*/
// 这个函数签名就显得不那么杂乱，函数名、参数列表和返回值类型都离得很近，看起来跟没有那么多 trait bounds 的函数很像。

// 返回实现了 trait 的类型
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// 不过这只适用于返回单一类型的情况。例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就行不通：
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// 使用 trait bound 有条件地实现方法

// 示例 10-15：根据 trait bound 在泛型上有条件的实现方法
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
