#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;
    let copy_of_c = c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    println!("ref_c1 not equals copy_of_c: {}", *ref_c1 == copy_of_c);

    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _ } = point;

        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let Point { x: _, y: ref mut mutable_ref_to_y } = mutable_point;

        *mutable_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable point is ({}, {})",
             mutable_point.x,
             mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}
