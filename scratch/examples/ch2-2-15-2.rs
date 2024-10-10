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

const NUM: usize = 2000000;

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

fn singl_thread() {
    let (mut v1, mut v2) = randomse_vec();

    let start = std::time::Instant::now();

    v1.sort();
    v2.sort();

    let end = start.elapsed();
    println!(
        "single_thread: {}.{:03}秒",
        end.as_secs(),
        end.subsec_micros() / 1_000_000
    );
}

fn mutlti_thread() {
    let (mut v1, mut v2) = randomse_vec();

    let start = std::time::Instant::now();

    let handler1 = std::thread::spawn(move || {
        v1.sort();
        v1
    });
    let handler2 = std::thread::spawn(move || {
        v2.sort();
        v2
    });

    let _v1 = handler1.join().unwrap();
    let _v2 = handler2.join().unwrap();
    let end = start.elapsed();
    println!(
        "multi_thread: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    )
}

fn main() {
    singl_thread();
    mutlti_thread();
}