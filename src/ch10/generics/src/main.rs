fn main() {
    // 示例 10-1：在一个数字列表中寻找最大值的函数
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let mut largest = &number_list[0];
    //
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //
    // println!("The largest number is {largest}");

    // 示例 10-2：寻找 两个 数字列表最大值的代码
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let mut largest = &number_list[0];
    //
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //
    // println!("The largest number is {largest}");
    //
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //
    // let mut largest = &number_list[0];
    //
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //
    // println!("The largest number is {largest}");

    // 示例 10-3：抽象后的寻找两个数字列表最大值的代码
    // fn largest(list: &[i32]) -> &i32 {
    //     let mut largest = &list[0];
    //
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //
    //     largest
    // }
    //
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {result}");
    //
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // 示例 10-4：两个函数，不同点只是名称和签名类型
    // fn largest_i32(list: &[i32]) -> &i32 {
    //     let mut largest = &list[0];
    //
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //
    //     largest
    // }
    //
    // fn largest_char(list: &[char]) -> &char {
    //     let mut largest = &list[0];
    //
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //
    //     largest
    // }
    //
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest_i32(&number_list);
    // println!("The largest number is {result}");
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest_char(&char_list);
    // println!("The largest char is {result}");

    // 示例 10-5：一个使用泛型参数的 largest 函数定义，尚不能编译  largest 的函数体不能适用于 T 的所有可能的类型
    // fn largest<T>(list: &[T]) -> &T {
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // 示例 10-7：字段 x 和 y 的类型必须相同，因为它们都有相同的泛型类型 T
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }
    //
    // let wont_work = Point { x: 5, y: 4.0 };

    // 结构体定义中的泛型
    // 示例 10-8：使用两个泛型的 Point，这样 x 和 y 可能是不同类型
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }
    //
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // 示例 10-9：在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }
    //
    // impl<T> Point<T> {
    //     fn x(&self) -> &T {
    //         &self.x
    //     }
    // }
    //
    // let p = Point { x: 5, y: 10 };
    //
    // println!("p.x = {}", p.x());
    //
    // 示例 10-10：构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块  这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。
    // impl Point<f32> {
    //     fn distance_from_origin(&self) -> f32 {
    //         (self.x.powi(2) + self.y.powi(2)).sqrt()
    //     }
    // }

    // 示例 10-11：方法使用了与结构体定义中不同类型的泛型
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 泛型代码的性能
    // 好消息是泛型并不会使程序比具体类型运行得慢。 Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
    // 在这个过程中，编译器所做的工作正好与示例 10-5 中我们创建泛型函数的步骤相反。编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。
}
