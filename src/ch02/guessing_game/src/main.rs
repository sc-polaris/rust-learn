// 示例 2-1：获取用户猜测并打印的代码

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 处理无效输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!")
            }
            Ordering::Greater => {
                println!("You big!");
            }
            Ordering::Equal => {
                println!("Too win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_println() {
        let x = 5;
        let y = 10;
        println!("x = {x} and y + 2 = {}", y + 2)
    }
}
