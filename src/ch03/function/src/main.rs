fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(5, 'h');

    // let y = 6;
    // let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值
    // let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // 在 five 函数中没有函数调用、宏、甚至没有 let 语句 —— 只有数字 5。这在 Rust 中是一个完全有效的函数。注意，也指定了函数返回值的类型，就是 -> i32。
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
