// use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过 use crate::garden::vegetables::Asparagus;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
