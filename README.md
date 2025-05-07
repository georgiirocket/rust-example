## Variables, constants

```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //Not can change
    let mut x = 5; //Can change
    let m = 6; // Not can change

    println!("Const {}", THREE_HOURS_IN_SECONDS);
    println!("This value is {}", x);
    println!("This m is {}", m);
    x = 6;
    println!("This value is {}", x);
}
```