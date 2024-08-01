fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 代码中的条件 必须 是 bool 值。
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // error
    // let number = if condition { 5 } else {"six" };

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break 关键字返回值 counter * 2
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // 循环标签：在多个循环之间消除歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while 条件循环
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!");

    // 使用 for 遍历集合
    // 使用 while 循环遍历集合中的元素
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // 使用 for 循环遍历集合中的元素
    for element in a {
        println!("the value is: {element}");
    }

    // 使用 for 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，rev，用来反转 range。
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
