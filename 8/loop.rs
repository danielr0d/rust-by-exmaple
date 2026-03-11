fn main() {
    let mut count = 0u32;

    println!("lets count til infinity :3");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("ok, dats enough");

            break;
        }
    }
}
