fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 常量 constants
    // Rust 对常量的命名约定是在单词之间使用全大写加下划线
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 我们可以定义一个与之前变量同名的新变量 Rustacean 们称之为第一个变量被第二个 隐藏（Shadowing） 了
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /*
       隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。
       通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。

       mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
    */
    // let spaces = "    ";
    // let spaces = spaces.len();

    // let mut spaces = "    ";
    // spaces = spaces.len();
}
