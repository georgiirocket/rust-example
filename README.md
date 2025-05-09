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

## Functions

### Simple [Functions]

```rust
fn main() {
    print_fn();
}

//Snake case
fn print_fn() {
    println!("Hello!!!");
}
```

### Fn with params [Functions]

```rust
fn main() {
    print_number(10);
}

fn print_number(x: i32) {
    println!("Number: {}", x);
}
```

### Fn with some params [Functions]

```rust
fn main() {
    print_labeled_measurement(10, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Expressions [Functions]

```rust
fn main() {
    let a: i64 = {
        let x: i64 = 20;
        x + 1 //Not use ; return value
    };

    println!("A = {}", a);
}
```

### Return value [Functions]

```rust
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
```

## Conditional expressions

```rust
fn main() {
    let x: i64 = 5;

    if x > 2 {
        println!("Number is {}", x)
    }
}
```

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### With expression [Conditional expressions]
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## Loop

### Infinity loop [Loop]
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

### Return value from loop [Loop]
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

### Cycle labels [Loop]
```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### While [Loop]
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

### For [Loop]
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

## Links

### Unchangeable [Links]
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    change(&s);

    println!("{s}");
}
fn change(s: &String) {
    println!("{}", s);
}
```

### Changeable [Links]
```rust
fn main() {
    let mut s = String::from("hello");  // s comes into scope

    change(&mut s);

    println!("{s}");
}
fn change(s: &mut String) {
    s.push_str(", world");
}
```