use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("The first element of slice is: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized with same values
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of array: {}", xs[0]);
    println!("second element of array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slice can point to a sectoin of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bounds indexing yields a panic
    println!("{}", xs[5]);
}


