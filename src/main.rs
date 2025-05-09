fn main() {
    let mut s = String::from("hello");  // s comes into scope

    change(&mut s);

    println!("{s}");
}
fn change(s: &mut String) {
   s.push_str(", world");
}