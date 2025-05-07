fn main() {
    let five: i64 = get_five();
    let six: i64 = get_six();

    println!("Number = {}", five);
    println!("Number = {}", six);
}

fn get_five() -> i64 {
    5 // Not use ;
}

fn get_six() -> i64 {
    return 6;
}