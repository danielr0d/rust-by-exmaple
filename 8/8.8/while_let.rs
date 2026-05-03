let mut optional = Some(0);

loop {
    match optional {
        Some(i) => {
            if i > 9 {
                println!("greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}. try again.", i);
            }
        },
        _ => { break ; }
    }
}
