## Variables

### let, const

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

### Shading

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

### Number

```rust
fn main() {
    let x: i64 = 5;
    let y: f64 = 3.05;

    println!("X: {}", x);
    println!("Y: {}", y);
}
```

### Boolean

```rust
fn main() {
    let t: bool = true;

    println!("T: {}", t);
}
```

### Symbol char

```rust
fn main() {
    let t: char = 'I';

    println!("T: {}", t);
}
```

### Composite type

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

### Array

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Item {}", a[1]);
}
```

### String

```rust
fn main() {
    let a: &'static str = "Test row";

    println!("Item {}", a);
}
```

## Functions

### Simple

```rust
fn main() {
    print_fn();
}

//Snake case
fn print_fn() {
    println!("Hello!!!");
}
```

### Fn with params

```rust
fn main() {
    print_number(10);
}

fn print_number(x: i32) {
    println!("Number: {}", x);
}
```

### Fn with some params

```rust
fn main() {
    print_labeled_measurement(10, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Expressions

```rust
fn main() {
    let a: i64 = {
        let x: i64 = 20;
        x + 1 //Not use ; return value
    };

    println!("A = {}", a);
}
```

### Return value

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

### With expression
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## Loop

### Infinity loop
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

### Return value from loop
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

### Cycle labels
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

### While
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

### For
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

### Unchangeable
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

### Changeable
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

## String slices
```rust
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];

    println!("{}", slice); //lo
}
```

```rust
fn main() {
    let s = String::from("hello world");

    let first_w = first_word(&s);

    println!("{}", first_w); //hello
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

## Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("<EMAIL>"),
        sign_in_count: 1,
    };

    println!("Username: {}", user.username)
}
```

### Copy struct

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("<EMAIL>"),
        sign_in_count: 1,
    };

    println!("Username: {}", user.username)
}
```

### Tuple structures

```rust
struct Color(i32, i32, i32);


fn main() {
    let user = Color(1, 2, 3);

    println!("Username: {}", user.1)
}
```

### Unit-like structures: structures without fields
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Methods
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn product(&mut self, number: u32) {
        self.width *= number;
        self.height *= number;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let mut  rectangle = Rectangle { width: 30, height: 50 };
    rectangle.product(2);

    let rect2 = Rectangle { width: 10, height: 40 };

    println!("Can hold: {}", rect2.can_hold(&rectangle));
    println!("Area: {}", rectangle.area());
}
```

### Associated functions

```rust
struct Rectangle {
    width: u32,
    height: u32,

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn product(&mut self, number: u32) {
        self.width *= number;
        self.height *= number;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height, }
    }
}


fn main() {
    let mut  rectangle = Rectangle { width: 30, height: 50 };
    rectangle.product(2);

    let rect2 = Rectangle::new(10, 40);

    println!("Can hold: {}", rect2.can_hold(&rectangle));
    println!("Area: {}", rectangle.area());
}
```

### Some blocks impl

```rust
struct Rectangle {
    width: u32,
    height: u32,

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn product(&mut self, number: u32) {
        self.width *= number;
        self.height *= number;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height, }
    }
}


fn main() {
    let mut  rectangle = Rectangle { width: 30, height: 50 };
    rectangle.product(2);

    let rect2 = Rectangle::new(10, 40);

    println!("Can hold: {}", rect2.can_hold(&rectangle));
    println!("Area: {}", rectangle.area());
}
```

## Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("X: {}", home.address)
}

```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### Option

```rust

fn main() {
    let mut  absent_number: Option<i32> = None;

    absent_number = Some(1);

    if absent_number.is_some() {
        println!("{:?}", absent_number);
    }
}

```

### Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let x = value_in_cents(Coin::Quarter);

    println!("x = {}", x);
}

```

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let x = value_in_cents(Coin::Penny);

    println!("x = {}", x);
}

```

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



fn main() {
    let five = Some(5);
    let six = plus_one(five);
}

```

```rust
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

```

```rust
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}


fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

```

### If let

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

```

## Vector
```rust
fn main() {
    let mut  v: Vec<i32> = vec![1, 2, 3];
    v.push(32);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }
}

```

## String

```rust
fn main() {
    let mut  st = String::from("hello");
    st.push_str(" world");
    
    println!("{}", st);
}

```

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    
    println!("s = {}", s);
}

```

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    for c in s.chars() {
        println!("{c}");
    }
}

```

## Hash map

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

```

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}'s score is {}", team_name, score);
}

```

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

```