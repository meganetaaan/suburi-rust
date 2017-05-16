fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Immutable vectors can't grow
    // collected_iterator.push(0);

    println!("Iterator size: {}", xs.len());

    println!("Second elem: {}", xs[1]);

    println!("Pop last elem: {:?}", xs.pop());

    println!("Fourth elem: {}", xs[3]);
}
