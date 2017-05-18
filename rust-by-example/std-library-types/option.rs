fn checked_division(devidend: i32, divisor: i32) -> Option<i32>{
    if divisor == 0 {
        None
    } else {
        Some(devidend / divisor)
    }
}

fn try_division (devidend: i32, divisor: i32) {
    match checked_division(devidend, divisor) {
        None => println!("{} / {} failed!", devidend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", devidend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(2, 0);

    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
