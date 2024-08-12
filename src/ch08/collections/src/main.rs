mod test {

    #[test]
    fn vec() {
        // 新建 vector

        // 示例 8-1：新建一个空的 vector 来储存 i32 类型的值
        // let v: Vec<i32> = Vec::new();

        // 示例 8-2：新建一个包含初值的 vector
        // let v = vec![1, 2, 3];

        // 更新 vector
        // 示例 8-3：使用 push 方法向 vector 增加值
        // let mut v = Vec::new();
        //
        // v.push(5);
        // v.push(6);
        // v.push(7);
        // v.push(8);

        // 读取 vector 的元素
        // 列表 8-4：使用索引语法或 get 方法来访问 vector 中的项
        // let v = vec![1, 2, 3, 4, 5];
        //
        // let third: &i32 = &v[2];
        // println!("The third element is {third}");
        //
        // let third: Option<&i32> = v.get(2);
        // match third {
        //     Some(third) => println!("The third element is {third}"),
        //     None => println!("There is no third element."),
        // }

        // 示例 8-5：尝试访问一个包含 5 个元素的 vector 的索引 100 处的元素
        // let does_not_exist = &v[100];
        // let does_not_exist = v.get(100);

        // 示例 8-6：在拥有 vector 中项的引用的同时向其增加一个元素
        // let mut v = vec![1, 2, 3, 4, 5];
        //
        // let first = &v[0];
        //
        // v.push(6);
        //
        // println!("The first element is: {first}");

        /*
           示例 8-6 中的代码看起来应该能够运行：为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，
           在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。
        */

        // 遍历 vector 中的元素
        // 示例 8-7：通过 for 循环遍历 vector 的元素并打印
        // let v = vec![100, 32, 57];
        // for i in &v {
        //     println!("{i}");
        // }
        // 示例 8-8：遍历 vector 中元素的可变引用
        // let mut v = vec![100, 32, 57];
        // for i in &mut v {
        //     *i += 50;
        // }

        // 使用枚举来储存多种类型
        // 示例 8-9：定义一个枚举，以便能在 vector 中存放不同类型的数据
        // enum SpreadsheetCell {
        //     Int(i32),
        //     Float(f64),
        //     Text(String),
        // }
        //
        // let row = vec![
        //     SpreadsheetCell::Int(3),
        //     SpreadsheetCell::Text(String::from("blue")),
        //     SpreadsheetCell::Float(10.12),
        // ];

        // 丢弃 vector 时也会丢弃其所有元素
        // 示例 8-10：展示 vector 和其元素于何处被丢弃
        // {
        //     let v = vec![1, 2, 3, 4];
        //
        //     // do stuff with v
        // } // <- v goes out of scope and is freed here
    }

    #[test]
    fn string() {
        // 新建字符串
        // 示例 8-11：新建一个空的 String
        // let mut s = String::new();

        // 示例 8-12：使用 to_string 方法从字符串字面值创建 String
        // let data = "initial contents";
        //
        // let s = data.to_string();
        //
        // // 该方法也可直接用于字符串字面值：
        // let s = "initial contents".to_string();

        // 示例 8-13：使用 String::from 函数从字符串字面值创建 String       等同于使用 to_string。
        // let s = String::from("initial contents");

        // 示例 8-14：在字符串中储存不同语言的问候语
        // let hello = String::from("السلام عليكم");
        // let hello = String::from("Dobrý den");
        // let hello = String::from("Hello");
        // let hello = String::from("שלום");
        // let hello = String::from("नमस्ते");
        // let hello = String::from("こんにちは");
        // let hello = String::from("안녕하세요");
        // let hello = String::from("你好");
        // let hello = String::from("Olá");
        // let hello = String::from("Здравствуйте");
        // let hello = String::from("Hola");

        // 更新字符串
        // 示例 8-15：使用 push_str 方法向 String 附加字符串 slice
        // let mut s = String::from("foo");
        // s.push_str("bar")

        // 示例 8-16：将字符串 slice 的内容附加到 String 后使用它
        // let mut s1 = String::from("foo");
        // let s2 = "bar";
        // s1.push_str(s2);
        // println!("s2 is {s2}");

        // 示例 8-17：使用 push 将一个字符加入 String 值中
        // let mut s = String::from("lo");
        // s.push('l');

        // 示例 8-18：使用 + 运算符将两个 String 值合并到一个新的 String 值中
        // let s1 = String::from("Hello, ");
        // let s2 = String::from("world!");
        // let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

        // 如果想要级联多个字符串，+ 的行为就显得笨重了
        // let s1 = String::from("tic");
        // let s2 = String::from("tac");
        // let s3 = String::from("toe");
        //
        // let s = s1 + "-" + &s2 + "-" + &s3;

        // 可以使用 format! 宏：   宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
        // let s1 = String::from("tic");
        // let s2 = String::from("tac");
        // let s3 = String::from("toe");
        //
        // let s = format!("{s1}-{s2}-{s3}");

        // 索引字符串
        // 示例 8-19：尝试对字符串使用索引语法   错误和提示说明了全部问题：Rust 的字符串不支持索引。
        // let s1 = String::from("hello");
        // let h = s1[0];

        // 内部表现 String 是一个 Vec<u8> 的封装
        // 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间（O(1)）。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。

        // 字符串 slice
        // 如果你真的希望使用索引创建字符串 slice 时，Rust 会要求你更明确一些。为了更明确索引并表明你需要一个字符串 slice，相比使用 [] 和单个值的索引，可以使用 [] 和一个 range 来创建含特定字节的字符串 slice：
        // let hello = "Здравствуйте";
        //
        // let s = &hello[0..4]; // 它包含字符串的头四个字节。早些时候，我们提到了这些字母都是两个字节长的，所以这意味着 s 将会是 “Зд”。
        // 如果获取 &hello[0..1] 会发生什么呢？答案是：Rust 在运行时会 panic，就跟访问 vector 中的无效索引时一样：

        // 遍历字符串的方法
        // for c in "Зд".chars() {
        //     println!("{c}");
        // }
        // for b in "Зд".bytes() {
        //     println!("{b}");
        // }

        /*
           称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 或字符
           串 slice &str 类型，而不特指其中某一个。虽然本部分内容大多是关于 String 的，不过这两个类型在 Rust 标准库中都被广泛使用，String 和字符串 slices 都是 UTF-8 编码的。
        */
    }

    #[test]
    fn hash_map() {
        // 新建一个哈希 map
        // 示例 8-20：新建一个哈希 map 并插入一些键值对
        // use std::collections::HashMap;
        // let mut scores = HashMap::new();
        //
        // scores.insert(String::from("Blue"), 10);
        // scores.insert(String::from("Yellow"), 50);

        // 访问哈希 map 中的值
        // 示例 8-21：访问哈希 map 中储存的蓝队分数
        // 这里，score 是与蓝队分数相关的值，应为 10。get 方法返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None。
        // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零。
        // let team_name = String::from("Blue");
        // let score = scores.get(&team_name).copied().unwrap_or(0);

        // for (key, value) in &scores {
        //     println!("{key}: {value}");
        // }

        // 哈希 map 和所有权
        // 示例 8-22：展示一旦键值对被插入后就为哈希 map 所拥有
        // use std::collections::HashMap;
        //
        // let field_name = String::from("Favorite color");
        // let field_value = String::from("Blue");
        //
        // let mut map = HashMap::new();
        // map.insert(field_name, field_value);
        // 这里 field_name 和 field_value 不再有效，
        // 尝试使用它们看看会出现什么编译错误！

        // 更新哈希 map
        // 示例 8-23：替换以特定键储存的值
        // use std::collections::HashMap;
        //
        // let mut scores = HashMap::new();
        //
        // scores.insert(String::from("Blue"), 10);
        // scores.insert(String::from("Blue"), 25); // 这会打印出 {"Blue": 25}。原始的值 10 则被覆盖了。
        //
        // println!("{scores:?}");

        // 示例 8-24：使用 entry 方法只在键没有对应一个值时插入
        // use std::collections::HashMap;
        //
        // let mut scores = HashMap::new();
        // scores.insert(String::from("Blue"), 10);
        //
        // scores.entry(String::from("Yellow")).or_insert(50);
        // scores.entry(String::from("Blue")).or_insert(50);
        // // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。这比编写自己的逻辑要简明的多，另外也与借用检查器结合得更好。
        //
        // println!("{scores:?}");

        // 示例 8-25：通过哈希 map 储存单词和计数来统计出现次数
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1
        }

        println!("{map:?}");

        // 哈希函数
        /*
           HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。然而这并不是可用的最快的算法，
           不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了
           BuildHasher trait 的类型。第十章会讨论 trait 和如何实现它们。你并不需要从头开始实现你自己的 hasher；crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。
        */
    }
}

fn main() {}
