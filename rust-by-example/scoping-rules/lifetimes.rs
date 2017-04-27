fn main() {
    let i = 3;

    {
        let borrow1 = &i;

        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}
