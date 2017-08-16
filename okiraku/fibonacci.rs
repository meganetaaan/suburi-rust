struct Fibonacci {
    a: i64,
    b: i64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let x = self.a;
        self.a = self.b;
        self.b += x;
        Some(x)
    }
}

fn main() {
    let mut fibo = Fibonacci::new();
    // F40から10このフィボナッチ数を求める
    let xs: Vec<i64> = fibo.skip(40).take(10).collect();
    println!("{:?}", xs);
    fibo = Fibonacci::new();
    // 300,000,000 未満で最も大きいフィボナッチ数Fnを求める
    println!("{}", fibo.take_while(|&x| x < 300_000_000).last().unwrap());
    // 300,000,000 未満のフィボナッチ数の総和を求める
    fibo = Fibonacci::new();
    let a: i64 = fibo.take_while(|&x| x < 300_000_000).sum();
    println!("{}", a);
    // 300,000,000 未満のフィボナッチ数の偶数項の総和を求める
    fibo = Fibonacci::new();
    let b: i64 = fibo.filter(|&x| x % 2 == 0)
        .take_while(|&x| x < 300_000_000)
        .sum();
    println!("{}", b);
}
