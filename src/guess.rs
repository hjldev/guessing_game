struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("请输入1到100之间到数字");
        }

        Guess {
            value,
        }
    }
}