fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("matched {:?}!", i);
    }

    if let Somme(i) = letter {
        println!("matched {:?}", i);
    } else {
        println!("didnt match a number. lets go with a latter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("matched {:?}!", i);
    } else if i_like_letters {
        println!("didint match a number. lets go with a letter! :3");
    } else {
        println!("i dont like letters. lets go with an emotion :) !");
    }
}
