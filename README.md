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

    // Error
    // let mut spaces = "   ";
    // spaces = spaces.len();
}

```

## Data types

### Number [Data types]

```rust
fn main() {
    let x: i64 = 5;
    let y: f64 = 3.05;

    println!("X: {}", x);
    println!("Y: {}", y);
}
```

### Boolean [Data types]

```rust
fn main() {
    let t: bool = true;

    println!("T: {}", t);
}
```

### Symbol char [Data types]

```rust
fn main() {
    let t: char = 'I';

    println!("T: {}", t);
}
```

### Composite type [Data types]

```rust
fn main() {
    let tup: (i64, f64, u8) = (32, 3.05, 1);

    let (x, y, z) = tup;

    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);

    println!("X: {}", tup.0);
    println!("Y: {}", tup.1);
    println!("Z: {}", tup.2);
}
```

### Array [Data types]

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Item {}", a[1]);
}
```

### String [Data types]

```rust
fn main() {
    let a: &'static str = "Test row";

    println!("Item {}", a);
}
```