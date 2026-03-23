fn main() {
    let names = vec!["Boc", "Danieo", "Ferris"];

    for name in names.iter() {
        match name {
            &"Danieo" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}
