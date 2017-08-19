fn sieve(n: usize) -> Vec<usize> {
    let mut ps: Vec<usize> = vec![2];
    let mut xs: Vec<bool> = vec![true; n / 2];
    let mut x = 3;
    while x * x <= n {
        let mut y = (x - 3) / 2;
        if xs[y] {
            ps.push(x);
            y += x;
            while y < xs.len() {
                xs[y] = false;
                y += x;
            }
        }
        x += 2;
    }
    while x <= n {
        if xs[(x - 3) / 2] {
            ps.push(x);
        }
        x += 2;
    }
    ps
}

fn sieve2(n: i32) -> Vec<i32> {
    let mut buff: Vec<i32> = (2..n + 1).collect();
    let mut ps: Vec<i32> = vec![];
    loop {
        let p = buff[0];
        ps.push(p);
        if p * p > n {
            break;
        }
        buff = buff.into_iter().filter(|x| x % p != 0).collect();
    }
    ps.append(&mut buff);
    ps
}

fn main() {
    // 素数
    println!("{:?}", sieve(100));
    println!("{:?}", sieve(1000));
    println!("{:?}", sieve2(100));
    println!("{:?}", sieve2(1000));
}
