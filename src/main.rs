use std::io;

use rand::Rng;
use std::cmp::Ordering;

mod guess;

fn main() {
    println!("游戏开始！");
    println!("请输入你的猜想数字");

    let people_num = 2;

    let mut guess_count = 0;

    let mut max_number = 100;
    let mut min_number = 1;

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // println!("请输入你的猜想数字");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("请输入1到100之间到数字");
            continue;
        }

        guess_count += 1;

        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("结果在 {} 跟 {} 之间", guess, max_number);
                min_number = guess;
            }
            Ordering::Greater => {
                println!("结果在 {} 跟 {} 之间", min_number, guess);
                max_number = guess;
            }
            Ordering::Equal => {
                break;
            }
        }
    }

    let people = guess_count % people_num;
    println!("{}号玩家赢得游戏", people);
}
