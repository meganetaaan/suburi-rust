fn create_box () {
    // Allocate an Integer on the heap
    let _box1 = Box::new(3i32);

    // _box1 is destroyed here. and memory gets freed
}

fn main () {
    // Allocate an Integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        let _box3 = Box::new(4i32);
        // _box3 is destroyed here. and memory gets freed
    }

    // Creating lots of boxed just for fun.
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // _box2 is destroyed here. and memory gets freed
}

