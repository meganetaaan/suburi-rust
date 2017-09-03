use std::time::Instant;
use std::collections::hash_map::HashMap;

fn tarai(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tarai(tarai(x - 1, y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))
    }
}

fn tarai_lazy(x: i32, y: i32, z: &Fn() -> i32) -> i32 {
    if x <= y {
        y
    } else {
        let zz = z();
        tarai_lazy(tarai_lazy(x - 1, y, z), tarai_lazy(y - 1, zz, &|| x), &|| {
            tarai_lazy(zz - 1, x, &|| y)
        })
    }
}

fn tak(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        z
    } else {
        tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y))
    }
}

fn tak_memo(x: i32, y: i32, z: i32, m: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    let key = (x, y, z);
    match m.get(&key) {
        Some(&v) => v,
        None => {
            if x <= y {
                m.insert(key, z);
                z
            } else {
                let v = tak_memo(
                    tak_memo(x - 1, y, z, m),
                    tak_memo(y - 1, z, x, m),
                    tak_memo(z - 1, x, y, m),
                    m,
                );
                m.insert(key, v);
                v
            }
        }
    }
}

fn main() {
    {
        let start = Instant::now();
        println!("{}", tarai(14, 7, 0));
        let end = start.elapsed();
        println!(
            "{},{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }
    {
        let start = Instant::now();
        println!("{}", tarai_lazy(14, 7, &|| 0));
        let end = start.elapsed();
        println!(
            "{},{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }
    {
        let start = Instant::now();
        println!("{}", tak(22, 11, 0));
        let end = start.elapsed();
        println!(
            "{},{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }
    {
        let start = Instant::now();
        println!("{}", tak_memo(22, 11, 0, &mut HashMap::new()));
        let end = start.elapsed();
        println!(
            "{},{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }
}
