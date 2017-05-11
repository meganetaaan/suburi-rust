#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;

    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copied: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! original has lost its resources
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    println!("cloned: {:?}", cloned_pair);
}
