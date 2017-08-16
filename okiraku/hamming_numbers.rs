struct HammingNumbers {
    buff: Vec<i64>,
    m2: i64,
    m3: i64,
    m5: i64,
    i2: usize,
    i3: usize,
    i5: usize,
}

// 生成
impl HammingNumbers {
    fn new() -> HammingNumbers {
        HammingNumbers {
            buff: vec![],
            m2: 1,
            m3: 1,
            m5: 1,
            i2: 0,
            i3: 0,
            i5: 0,
        }
    }
}

// イテレータの実装
impl Iterator for HammingNumbers {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let m = std::cmp::min(std::cmp::min(self.m2, self.m3), self.m5);
        self.buff.push(m);
        while self.m2 <= m {
            self.m2 = self.buff[self.i2] * 2;
            self.i2 += 1;
        }
        while self.m3 <= m {
            self.m3 = self.buff[self.i3] * 3;
            self.i3 += 1;
        }
        while self.m5 <= m {
            self.m5 = self.buff[self.i5] * 5;
            self.i5 += 1;
        }
        Some(m)
    }
}

fn main() {
    // ハミング数
    let hn = HammingNumbers::new();
    println!("{:?}", hn.take(100).collect::<Vec<i64>>());
}
