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

// TwinPrimes
struct TwinPrimes {
    ps: Primes,
    prev: i32,
}

impl TwinPrimes {
    fn new() -> TwinPrimes {
        TwinPrimes {
            ps: Primes::new(),
            prev: 1,
        }
    }
    fn reset(&mut self) {
        self.ps.reset();
        self.prev = 1;
    }
}

impl Iterator for TwinPrimes {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.ps.next().unwrap();
            if p - self.prev == 2 {
                let q = self.prev;
                self.prev = p;
                return Some((q, p));
            }
            self.prev = p;
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

    let mut twinps = TwinPrimes::new();
    for _ in 0..100 {
        print!("{:?}", twinps.next().unwrap());
    }
    println!("");
    twinps.reset();
    println!(
        "{:?}",
        twinps.take_while(|x| x.1 < 3_000_000).last().unwrap()
    );
}
