struct XOR64 {
    x: u64,
}

impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }

    /// 疑似乱数生成関数
    fn next(&mut self) -> u64 {
        let x = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x >> 7);
        let x = x ^ (x << 17);
        self.x = x;
        x
    }
}

const NUM: usize = 200000000;

fn randomse_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    let mut generator = XOR64::new(1234);

    for _ in 0..NUM {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }
    (v1, v2)
}

fn main() {
    let (r1, r2) = randomse_vec();
    println!("{:?}", r1);
    println!("{:?}", r2);
}