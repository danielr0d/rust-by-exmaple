#[derive(Debug)]
struct Person<'a'> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "danieo";
    let age = 27;
    let danieo = Person { name, age };

    //Tal do pretty print 
    println!("{:?}", danieo);
}
