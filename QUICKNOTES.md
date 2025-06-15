# ToC 
- [ToC](#toc)
- [Install rustup](#install-rustup)
  - [Rustup basic info](#rustup-basic-info)
  - [Updating](#updating)
  - [Local doc](#local-doc)
  - [Cargo commands](#cargo-commands)
- [Language](#language)
  - [Arrays](#arrays)
  - [Tuples](#tuples)
  - [Borrows](#borrows)
  - [Strings](#strings)
  - [Mutable and immutable references](#mutable-and-immutable-references)
  - [Structs](#structs)
    - [Field update syntax](#field-update-syntax)
    - [Tuple structs](#tuple-structs)
    - [Methods](#methods)
    - [Enums](#enums)
    - [Option](#option)
    - [Match construct](#match-construct)
    - [Concise control flow with if let](#concise-control-flow-with-if-let)
  - [Crates and modules](#crates-and-modules)
    - [Declaring modules](#declaring-modules)
    - [Declaring submodules](#declaring-submodules)
    - [Path to the modules](#path-to-the-modules)
    - [Private vs public](#private-vs-public)
    - [Use keyword](#use-keyword)
      - [`as` keyword](#as-keyword)
      - [re-exporting](#re-exporting)
    - [Modules hierarchy](#modules-hierarchy)
    - [Paths](#paths)
      - [Absolute](#absolute)
      - [Relative](#relative)
    - [public in context of structs and enums](#public-in-context-of-structs-and-enums)
    - [External packages](#external-packages)
    - [Nested paths](#nested-paths)
    - [Glob operator \*](#glob-operator-)
  - [Collections](#collections)
    - [Vector](#vector)
    - [Strings](#strings-1)
      - [Creation](#creation)
      - [Concatenation](#concatenation)
      - [Indexing](#indexing)
      - [Slicing](#slicing)
      - [Iterating over](#iterating-over)
    - [Hash maps](#hash-maps)
      - [Creation](#creation-1)
      - [Element access](#element-access)
      - [Iterating over](#iterating-over-1)
      - [Elements ownership](#elements-ownership)
      - [Entry API](#entry-api)
  - [Error handling](#error-handling)
    - [Panic behavior](#panic-behavior)
    - [Result](#result)
  - [Generic types, traits and lifetimes](#generic-types-traits-and-lifetimes)
    - [Generics](#generics)
    - [Traits](#traits)
      - [Default implementation](#default-implementation)
      - [Traits as a parameters](#traits-as-a-parameters)
      - [Where clause](#where-clause)
      - [Return type traits](#return-type-traits)
    - [Lifetime annotation syntax](#lifetime-annotation-syntax)

# Install rustup

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Rustup basic info

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

## Updating

```
rustup update
```

## Local doc

```
rustup doc
```
## Cargo commands

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

`use` creates a shortcut **within a scope**. `use crate::fancy::disco::Ball` makes `Ball` accessible in a scope without prefix.
Follows privacy rules as normal paths.
Consider using idiomatic paths using `use` keyword, so that module is defined in usage of a function.

Prefer:
```rust
use crate::fancy::disco;
...
let x = disco::make_ball();
```
over:
```rust
use crate::fancy::disco::make_ball;
...
let x = make_ball{};
```
so that module name is explicit.

On the other hand when bringing structs, enums and other items with use it is idiomatic to specify full path.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
This an idiom that rust folks are using. But it has exception. If bringing two structs with the same name from different modules, we fallback to approach with functions

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```
#### `as` keyword 
To provide new name when defining paths with `use` the `as` keyword can be used

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

#### re-exporting
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
this make add_to_waitlist callable without need the front_of_house to be `pub`. Also now the call from external module will look like `restaurant::hosting::add_to_waitlist()`

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


### External packages
Defined in `Cargo.toml`, downloaded from `crates.io`

### Nested paths
instead of:
```rust
use std::cmp::Ordering;
use std::io;
```
can use
```rust
use std::{cmp::Ordering, io};
```

instead of:
```rust
use std::io;
use std::io::Write;
```
can use
```rust
use std::io::{self, Write};
```

### Glob operator *
Brings all public items defined in a path into scope
```rust
use std::collections::*;
```

## Collections

### Vector

```rust
let v: Vec<i32> = Vec::create();
```

### Strings
#### Creation
```rust
let s1 = String::from("Hello");
  let s2 = "initial contents".to_string();
```

#### Concatenation
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
The above example is simple but tricky at the same time.Second parameter of `+ operator` is ref and first is moved. It is because `+ operator` uses `add` method that is defined as
```rust
fn add(self, s: &str) -> String {
  ...
```
also there is a deref coercion which turns` &String` to `&str`

so `s1` is moved in, `s2` is taken by ref and ownership to modified s1 is returned as `s3`

`format!` macro can be used to construct `Strings` from other `Strings`
```rust

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");  

    let s = format!("{s1}-{s2}-{s3}");
```
#### Indexing
Characters in `Strings` cannot be indexed by `[] operator` as `Strings` are UTF-8 based so in some representations there does not have sense.
Also `[i]` access should have (O(1)) complexity which can't be guaranteed as string would need to be walked from beginning rendering it more like (O(n))

> [!Caution]
> ```rust
> let hello = String::from("Здравствуйте");
> let character = hello[0];
> ```
> Above will not compile.

#### Slicing
String can be sliced using `[a..b]` as other type slices but these have character boundary check, so trying to create slice in a middle of multibyte character will result in panic

```rust

let hello = String::from("Cześć");
let slice = &hello[0..4];

```
will result in
```
thread 'main' panicked at src/string.rs:89:19:
byte index 4 is not a char boundary; it is inside 'ś' (bytes 3..5) of `Cześć`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### Iterating over
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```
```
З
д
```

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```
```
208
151
208
180
```

### Hash maps
#### Creation
```rust
use std::collections::HashMap;
...
let mut scores: HashMap<String, i32> = HashMap::new();
```
Not in prelude so need to bring it into scope manually. No macro to create. 

#### Element access
```rust
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```
`get` method returns `Option<&V>`, `copied()` copy it to `Option<V>` and `unwrap_or(x)` set it to x if `Option` is `None`

#### Iterating over
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```
Iterates over tuple of key and value `(&K, &V)`

#### Elements ownership

If type hold in `HashMap` has a `Copy` trait it is copied inside. If not it is moved so using original is not valid.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get! 
```

#### Entry API

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
```
Insert value if key does not exists. Returns `Entry` enum.

## Error handling

### Panic behavior
To abort instead of stack unwind and exit on `panic!` set
```toml
[profile.release]
panic = 'abort'
```
in `Cargo.toml`

`panic!` macro crashes the program

### Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
`unwrap` returns `Ok` from `Result` or panics

`expect` same as `unwrap` but defines `panic!` message

`? operator` can be used to propagate `Err` up, so `?` means return Err(e) if Err(e)


```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

`Operator ?` can be chained like

```rust
File::open("hello.txt")?.read_to_string(&mut username)?;
```

This can also be used with `Option<T>` in simmilar manner. `Some(T)` or `None` will be returned.

## Generic types, traits and lifetimes

### Generics
Simmilar to templates in C++. Type placeholder in `<>` before function parameters or next to struct or enum name.

eg.:
```rust
fn largest<T>(list: &[T]) -> &T { ... }
...

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {    // this first <T> after impl is needed to tell compiler that <T> in Point is generic type, not concrete one
    fn x(&self) -> &T {
        &self.x
    }
}

// Specialization (method accessible only for f32)
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
 
enum Option<T> {
    Some(T),
    None,
}

```

To restrict types that can be used with T, need to define trait.

eg.:
```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { ... }
```

As in C++ templates, Rust with generics is monomorphizing types (replaces template/generic parameter with concrete type) during compilation, so generics does not have runtime performance overhead.

### Traits
Traits define common behaviour (like interface) for different types.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

> [!important]
> We can implement a trait on a type only if either the trait or the type, or both, are local to our crate.


#### Default implementation

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

...

impl Summary for NewsArticle {}  // use default implementation
```

#### Traits as a parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
Use `impl Trait` tell that any type implementing `Trait` is accepted

above is an equivalent to
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
Traits can be joined so that type in parameter has to implement two or more traits. Operator `+` is used for that.

```rust
pub fn notify(item: &(impl Summary + Display)) { ... } 
```
or
```rust
pub fn notify<T: Summary + Display>(item: &T) { ... } 
```

#### Where clause
Traits can be defined in `where` clause to clean up function declaration 
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  ...
```

#### Return type traits
Use `impl Trait` in return position to "something that implements Trait"
```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```
Calling code sees only Summary trait returned, not concrete type, even if `SocialPost` is returned

We can conditional implement a trait for type that implements other trait with syntax below.
```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

### Lifetime annotation syntax

`'` is used to annotate generic lifetime relationship (parameter name starts with it).

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

So as an example, we want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add it to each reference, as shown below
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> [!Note]
> Above function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.
