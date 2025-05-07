## Variables

### let, const [variables]

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

### Shading [variables]

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = 3;
        println!("The value of x in the inner scope is: {x}"); // 3
    }

    println!("The value of x is: {x}"); // 6
}

```