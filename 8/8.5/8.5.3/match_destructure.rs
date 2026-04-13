fn some_number() - Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        Some(n @ 42) => println!("the answer: {}!", n),
        Some(n)      => println!("not interesting... {}", n),
        _            => (),
    }
}
