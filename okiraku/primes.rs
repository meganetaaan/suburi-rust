struct Primes {
    primes: Vec<i32>,
    idx: usize,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: vec![2, 3],
            idx: 0,
        }
    }
    // 素数列をリセットする
    fn reset(&mut self) {
        self.idx = 0;
    }
}

impl Iterator for Primes {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.primes.len() {
            self.idx += 1;
            Some(self.primes[self.idx - 1])
        } else {
            let mut n = self.primes[self.primes.len() - 1] + 2;
            loop {
                if self.primes.iter().take_while(|&&x| x * x <= n).all(|&x| {
                    n % x != 0
                })
                {
                    self.primes.push(n);
                    self.idx += 1;
                    return Some(n);
                }
                n += 2;
            }
        }
    }
}

fn main() {
    let mut ps = Primes::new();
    for _ in 0..25 {
        print!("{} ", ps.next().unwrap());
    }
    println!("");
    ps.reset();
    for _ in 0..100 {
        print!("{} ", ps.next().unwrap());
    }
    println!("");
}
