

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
