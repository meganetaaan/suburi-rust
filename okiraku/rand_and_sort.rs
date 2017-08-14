struct Rand {
    seed: u32
}

const RAND_MAX: u32 = 0xffff_ffff;

impl Rand {
    fn new(x: u32) -> Rand {
        Rand {seed: x}
    }

    fn rand(&mut self) -> u32 {
        let x = self.seed as u64;
        self.seed = ((69069 * x + 1) & RAND_MAX as u64) as u32;
        self.seed
    }

    fn random(&mut self) -> f64 {
        (1.0 / (RAND_MAX as f64 + 1.0)) * self.rand() as f64
    }

    fn shuffle<T>(&mut self, buff: &mut [T]) {
        for i in 0 .. buff.len() {
            let j = (self.random() * buff.len() as f64) as usize;
            buff.swap(i, j);
        }
    }
}

fn bubble_sort<T: Ord>(buff: &mut [T]) {
    for i in 0 .. buff.len() {
        let mut j = buff.len() - 1;
        while j > i {
            if buff[j] < buff[j - 1] {
                buff.swap(j, j - 1);
            }
            j -= 1;
        }
    }
}

fn select_sort<T: Ord>(buff: &mut [T]) {
    for i in 0 .. buff.len() - 1{
        let mut n = i;
        for j in i + 1 .. buff.len() {
            if buff[j] < buff[n] {
                n = j;
            }
        }
        buff.swap(i, n);
    }
}

fn insert_sort<T: Ord>(buff: &mut [T]) {
    for i in 1 .. buff.len() {
        let mut j = i;
        while j > 0 && buff[j] < buff[j - 1] {
            buff.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn insert_sort1<T: Ord + Copy>(buff: &mut [T]) {
    for i in 1 .. buff.len() {
        let mut j = i;
        let temp = buff[i];
        while j > 0 && temp < buff[j - 1] {
            buff[j] = buff[j - 1];
            j -= 1;
        }
        buff[j] = temp;
    }
}

fn quick_sort<T: Ord + Copy>(buff: &mut [T]) {
    if buff.len() < 2 { return; }
    let pivot = buff[buff.len() / 2];
    let mut i = 0;
    let mut j = buff.len() - 1;
    loop {
        while pivot > buff[i] { i += 1; }
        while pivot < buff[j] { j -= 1; }
        if i >= j { break; }
        buff.swap(i, j);
        i += 1;
        j -= 1;
    }
    if i > 0 { quick_sort(&mut buff[.. i]); }
    if j < buff.len() - 1 { quick_sort(&mut buff[j + 1 ..]); }
}

const LIMIT: usize = 16;

fn select_pivot<T: Ord + Copy>(buff: &[T]) -> T {
    let a = buff[0];
    let b = buff[buff.len() / 2];
    let c = buff[buff.len() - 1];
    if a < b {
        if b < c {  
            b // a < b < c
        } else if a < c {
            c // a < c < b
        } else {
            a // c < a < b
        }
    } else {
        if a < c {
            a // b < a < c
        } else if b < c {
            c // b < c < a
        } else {
            b // c < b < a
        }
    }
}

fn quick_sort1<T: Ord + Copy>(buff: &mut[T]) {
    if buff.len() <= LIMIT {
        insert_sort1(buff);
        return;
    }
    let pivot = select_pivot(buff);
    let mut i = 0;
    let mut j = buff.len() - 1;
    loop {
        while pivot > buff[i] { i += 1; }
        while pivot < buff[j] { j -= 1; }
        if i >= j { break; }
        buff.swap(i, j);
        i += 1;
        j -= 1;
    }
    if i > 0 { quick_sort1(&mut buff[..i]); }
    if j < buff.len() - 1 { quick_sort1(&mut buff[j + 1 ..]); }
}

fn merge_sort<T: Ord + Copy>(buff: &mut [T]) {
    if buff.len() < 2 {
        return;
    } else if buff.len() == 2 {
        if buff[0] > buff[1] { buff.swap(0, 1); }
    } else {
        let mid = buff.len() / 2;
        merge_sort(&mut buff[.. mid]);
        merge_sort(&mut buff[mid ..]);
        // 前半部分をWorkに対比
        let mut work: Vec<T> = Vec::with_capacity(mid);
        for i in 0 .. mid { work.push(buff[i]); }
        let mut i = 0;
        let mut j = mid;
        let mut k = 0;
        while i < mid && j < buff.len() {
            if work[i] <= buff[j] {
                buff[k] = work[i];
                i += 1;
            } else {
                buff[k] = buff[j];
                j += 1;
            }
            k += 1;
        }
        while i < mid {
            buff[k] = work[i];
            i += 1;
            k += 1;
        }
    }
}

fn test(func: fn(&mut [i32]) -> (), rng: &mut Rand) {
    let mut buff: [i32; 20] = [0; 20];
    for i in 0 .. 20 { buff[i] = i as i32; }
    rng.shuffle(&mut buff);
    println!("{:?}", buff);
    func(&mut buff);
    println!("{:?}", buff);
    buff.reverse();
    println!("{:?}", buff);
    func(&mut buff);
    println!("{:?}", buff);
    func(&mut buff);
    println!("{:?}", buff);
}

fn main() {
    let mut rng = Rand::new(1);
    println!("----- buble sort -----");
    test(bubble_sort, &mut rng);
    println!("----- select sort -----");
    test(select_sort, &mut rng);
    println!("----- insert sort -----");
    test(insert_sort, &mut rng);
    println!("----- insert sort 1 -----");
    test(insert_sort1, &mut rng);
    println!("----- quick sort -----");
    test(quick_sort, &mut rng);
    println!("----- quick sort 1 -----");
    test(quick_sort1, &mut rng);
    println!("----- merge sort -----");
    test(merge_sort, &mut rng);
}
