use std::io;

fn main() {
    // 使用 parse 将 String 转换为数字时，必须增加类型注解
    let guess: u32 = "42".parse().expect("Not a number");

    // 标量类型
    // 整型
    /*
        长度	    有符号	无符号
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize

        isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的。

        数字字面值	                例子
        Decimal (十进制)	            98_222
        Hex (十六进制)	            0xff
        Octal (八进制)	            0o77
        Binary (二进制)	            0b1111_0000
        Byte (单字节字符)(仅限于u8)	b'A'
    */

    // 浮点型
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数值计算
    // addition
    let sum = 5 + 10;

    // subtraction
    let differenct = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;

    // 布尔型
    let t = true;
    let f: bool = false; // with explicit type annotation

    // 字符类型
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // 复合类型
    // 元组
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 模式结构
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // 我们也可以使用点号（.）后跟值的索引来直接访问它们
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // 数组类型
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // 可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 你还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
    let a = [3; 5];

    // 访问数组元素
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // 无效的数组元素访问
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
