fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = 3;
        println!("The value of x in the inner scope is: {x}"); // 3
    }

    println!("The value of x is: {x}"); // 6

    // Error
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
