enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    if Foo::Bar == a { 
    // ^-- this causes a compile-time error. us if let instead hihi :3
        println!("a is foobar");
    }
}
