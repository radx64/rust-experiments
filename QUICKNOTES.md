

### Install rustup

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Rustup basic info

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  `/home/radek/.rustup`

The Cargo home directory is located at:

  `/home/radek/.cargo`

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  `/home/radek/.cargo/bin`

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  `/home/radek/.profile`
  `/home/radek/.bashrc`

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

### Updating

```
rustup update
```

### Local doc

```
rustup doc
```
### Cargo commands

```
cargo new <project_name> - creates new project called <project_name>
cargo init            - convert directory to cargo project (code need to be moved to src subdir first)
cargo build           - builds debug version of a project
cargo build --release - builds release version of a project
cargo run             - build if needded and run project
cargo check           - check if project compiles
```

Cargo stores compilation results in `target/` directory

# Language
drop - function called when object goes out of scope (destructor? - not exactly, cant use Copy if drop is used)

`#[derive(Debug)]` trait that implements debug printing of a struct that define usage of this.
Then `println!()` can use `{:?}` or `{:#?}` to make type printable

## Arrays 
```rust
let a = [1, 2, 3, 4, 5];
```

## Tuples
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

## Borrows
  `&` - acts as a reference (borrows instance)

## Strings
  `str` - basic string slice type to which String is convertible

## Mutable and immutable references
  You can't have both mutable an immutable references (borrows) at the same time

## Structs
  When initializing struct fields inside of function there is a shorthand, if function parameter has a same name as struct field, field name can be ommited so instead `username: username`, you can write just `username` on struct parameter list.

```rust
fn build_user(email: String, username: String) -> User{
    User { active: true, username, email, sign_in_count: 1 }
}
```

### Field update syntax
```rust
let user2 = User {
    email: String::from("yetanotheremail@world.com"),
    username: user1.username.clone(),
    ..user1     // this is struct update syntax (fill rest fields same as in user1)
};
```

### Tuple structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

To destructurize tuple struct type name need to be provided
```rust
  let Point(x,y,z) = origin;
```

### Methods
Methods take `&self` as a first argument if operates on instance. If self is ommited it is called assoctiated function (something like static class methods in C++) so no instance of a type is needed to call them. `Type::method` syntax is used there when calling


### Enums
Enums are defined using `enum` keyword

```rust
enum IpAddrKind {
    V4,
    V6
}
```
Every enum value can store data assigned to value. This automatically generates a function that can be used to construct enum instance:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}
...
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Each enum value can store different types

```rust
  enum IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
  }

  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));
```
### Option
Something like std::optional in cpp but different.
Enum that stores `Some(T) or None`. Can be used with match construct for handling ok and nok cases/

```rust
  let some_number = Some(5);
  let some_char = Some('e');

  let absent_number: Option<i32> = None;
```
`None` requires type annotation


### Match construct
Used to map enum values to piece of code to run. Match are exhaustive.

```rust
match variable {
 1 => something,
 3 => something_else,
 other => otherwise(other)  // this can be swaped with _ (placeholder) if we don't want to use value
}
```
unit values `()` can be also returned from match

### Concise control flow with if let

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```
is equivalent to

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
```

`if let` can also use else statement which mates to _

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    else {
        println!("max is not configured");
    }
```

Sometimes you want early exit from function and let can have else for that
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America1"))
    }
    else {
        Some(format!("{state:?} is relatively new."))
    }
}
```
Above will early exit if coin is not quarter but if it is it will bring state into scope for further handling.


## Crates and modules

If project has `src/main.rs` it has a binary create. Binary crates **must** have `main` function.

If project has `src/lib.rs` it as a library create. Library crates **does not** have `main` function.

Crate is a single compilation unit.

### Declaring modules

`mod fancy;` - compiler will look for `mod fancy{}` then `src/fancy.rs` then `src/fancy/mod.rs`

### Declaring submodules

in `fancy.rs` `mod disco` - compiler will look for `mod disco{}` then `src/fancy/disco.rd` then `src/fancy/disco/mod.rs`

### Path to the modules

Type `Ball` in module disco would be found at `crate::fancy::disco::Ball`

### Private vs public

Code in the module is private by default. To make module public declare it with `pub mod` instead of `mod`. To make items public within a module use `pub` before declarations of these items.

### Use keyword

`use` creates a shortcut within a scope. `use crate::fancy::disco::Ball` makes `Ball` accessible in a scope without prefix.


### Modules hierarchy

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

creates
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
see `crate` root 

### Paths
#### Absolute
  Full path from create name (for external crates) or literal `create` for current create

#### Relative
  Starts from current module and uses self, super or identifier in current module

Identifiers are followed by `::`

### public in context of structs and enums
Public struct has a private fields unless fields are defined as public. If there is at least one private field, stuct can't be constructed and there is a need of some kind of factory method in struct itself to exists to construct object.

Public enum makes all variants of enum public.
