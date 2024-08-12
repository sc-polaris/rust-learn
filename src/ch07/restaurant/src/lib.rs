// 示例 7-1：一个包含了其他内置了函数的模块的 front_of_house 模块
// mod front_of_house {
//     // 模块公有并不使其内容也是公有的。模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码。因为模块是一个容器，只是将模块变为公有能做的其实并不太多；同时需要更深入地选择将一个或多个项变为公有。
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_order() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// 示例 7-8: 使用以 super 开头的相对路径从父目录开始调用函数
/*
   它模拟了厨师更正了一个错误订单，并亲自将其提供给客户的情况。back_of_house 模块中的定义的 fix_incorrect_order
   函数通过指定的 super 起始的 deliver_order 路径，来调用父模块中的 deliver_order 函数：
*/

// fn deliver_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
// }

// 示例 7-9: 带有公有和私有字段的结构体

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // 在夏天订购一个黑麦吐司作为早餐
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // 改变主意更换想要的面包类型
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     // 如果取消下一行的注释代码不能编译；
//     // 不允许查看或修改早餐附带的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// 示例 7-10: 设计公有枚举，使其所有成员公有
/*
   如果枚举成员不是公有的，那么枚举会显得用处不大；给枚举的所有成员挨个添加 pub 是很令人恼火的，因此枚举成员默认就是公有的。结构体通常使用时，
   不必将它们的字段公有化，因此结构体遵循常规，内容全部是私有的，除非使用 pub 关键字。
*/

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// 示例 7-11: 使用 use 将模块引入作用域

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// 示例 7-12: use 语句只适用于其所在的作用域

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// use crate::front_of_house::hosting;
//
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// 示例 7-13: 使用 use 将 add_to_waitlist 函数引入作用域，这并不符合习惯

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// use crate::front_of_house::hosting::add_to_waitlist;
//
// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// 示例 7-14: 将 HashMap 引入作用域的习惯用法
// 另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。

// use std::collections::HashMap;
//
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// 示例 7-15: 使用父模块将两个具有相同名称的类型引入同一作用域

// use std::fmt;
// use std::io;
//
// fn function1() -> fmt::Result {
//     // --snip--
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
// }

// 示例 7-16: 使用 as 关键字重命名引入作用域的类型

// use std::fmt::Result;
// use std::io::Result as IoResult;
//
// fn function1() -> Result {
//     // --snip--
// }
//
// fn function2() -> IoResult<()> {
//     // --snip--
// }

// 示例 7-17: 通过 pub use 使名称可从新作用域中被导入至任何代码
/*
   使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。如果想让其他人调用我们的代码时，
   也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：我们
   不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。
*/

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// pub use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// 嵌套路径来消除大量的 use 行

// use std::cmp::Ordering;
// use std::io;

// 示例 7-18: 指定嵌套的路径在一行中将多个带有相同前缀的项引入作用域
// use std::{cmp::Ordering, io};

// 示例 7-19: 通过两行 use 语句引入两个路径，其中一个是另一个的子路径
// use std::io;
// use std::io::Write;

// 示例 7-20: 将示例 7-19 中部分重复的路径合并为一个 use 语句   这一行便将 std::io 和 std::io::Write 同时引入作用域。
// use std::io::{self, Write};

// 通过 glob 运算符将所有的公有定义引入作用域
// 如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符：
// use std::collections::*;

// 示例 7-21: 声明 front_of_house 模块，其内容将位于 src/front_of_house.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
