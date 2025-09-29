# ToC 
- [ToC](#toc)
- [Rustup](#rustup)
  - [Install rustup](#install-rustup)
  - [Rustup basic info](#rustup-basic-info)
  - [Updating](#updating)
  - [Local doc](#local-doc)
- [Cargo](#cargo)
  - [Commands](#commands)
    - [Extending cargo](#extending-cargo)
  - [Customizing profiles](#customizing-profiles)
    - [Profile settings](#profile-settings)
      - [opt-level](#opt-level)
      - [debug](#debug)
      - [split-debuginfo](#split-debuginfo)
      - [strip](#strip)
      - [debug-assertions](#debug-assertions)
      - [overflow-checks](#overflow-checks)
      - [lto](#lto)
      - [panic](#panic)
      - [incremental](#incremental)
      - [codegen-units](#codegen-units)
      - [rpath](#rpath)
  - [Creating crate on crates.io](#creating-crate-on-cratesio)
  - [Updating a create on crates.io](#updating-a-create-on-cratesio)
  - [Deprecating old versions](#deprecating-old-versions)
  - [Undeprecating old versions](#undeprecating-old-versions)
  - [Workspaces](#workspaces)
- [Language](#language)
  - [Arrays](#arrays)
  - [Tuples](#tuples)
  - [Comments](#comments)
    - [Code comments](#code-comments)
    - [Documentation comments](#documentation-comments)
    - [Contained items comment](#contained-items-comment)
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
    - [Lifetime elision rules](#lifetime-elision-rules)
    - [Static lifetime](#static-lifetime)
  - [Testing](#testing)
    - [Useful macros](#useful-macros)
    - [Panics](#panics)
    - [Result\<T, E\> in tests](#resultt-e-in-tests)
    - [Running tests](#running-tests)
      - [Basic cargo usage](#basic-cargo-usage)
      - [Running tests by name](#running-tests-by-name)
      - [Ignoring tests](#ignoring-tests)
    - [Testing convention - Unit tests](#testing-convention---unit-tests)
    - [Testing private functions](#testing-private-functions)
    - [Testing convention - Integration tests](#testing-convention---integration-tests)
      - [Sharing common code between tests](#sharing-common-code-between-tests)
        - [Integration Tests for Binary Crates](#integration-tests-for-binary-crates)
  - [Program arguments](#program-arguments)
  - [File System](#file-system)
  - [Process exit code](#process-exit-code)
  - [Environment variables](#environment-variables)
  - [Standard error output stream](#standard-error-output-stream)
  - [Closures](#closures)
      - [Types](#types)
      - [Examples](#examples)
    - [Comparision to functions syntax](#comparision-to-functions-syntax)
    - [Closure that takes an ownership of the value](#closure-that-takes-an-ownership-of-the-value)
  - [Iterators](#iterators)
    - [Some iterators methods](#some-iterators-methods)
  - [Public reexports](#public-reexports)
  - [Smart pointers](#smart-pointers)
    - [Common smart pointers](#common-smart-pointers)
    - [Deref](#deref)
      - [Mutability and deref coercion](#mutability-and-deref-coercion)
    - [Drop](#drop)

# Rustup

## Install rustup

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
# Cargo 

## Commands

```
cargo new <project_name> - creates new project called <project_name>
cargo init               - convert directory to cargo project (code need to be moved to src subdir first)
cargo build              - builds debug version of a project
cargo build --release    - builds release version of a project
cargo doc                - builds documentation
cargo run                - build if needded and run project
cargo check              - check if project compiles
cargo test               - run tests target
cargo bench              - run benchmarks target
cargo install <binname>  - install binary target (application) from cargo.io
```

Cargo stores compilation results in `target/` directory

### Extending cargo

If binary is in $PATH is named `cargo-something` it can be run as `cargo something`.
Cargo commands can be listed using `cargo --list`

## Customizing profiles
Create `[profile.*]` section in Cargo.toml file to customize settings like opt-level.

More info [here](https://doc.rust-lang.org/cargo/reference/profiles.html).

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

### Profile settings

#### opt-level
`0` - no optimization <br>
`1` - basic optimizations <br>
`2` - some optimizations <br>
`3` - all optimizations <br>
`"s"` - optimize for size <br>
`"z"` - optimize for size but turn off loop vectorization <br>

#### debug
`0`, `false`, `"none"` - no debug info <br>
`"line-directives-only"` - line info directives only <br>
`"line-tables-only"` - line tables only <br>
`1` or `"limited"` - debug info without type oe variable level information, little more detailed as `line-tables-only` <br>
`2`, `true` or `"full"` - full debug info, default for dev profile <br>
More info [here](https://doc.rust-lang.org/rustc/codegen-options/index.html#debuginfo)

#### split-debuginfo
Controls if debug information is placed inside executable or next to it.

#### strip
Used to control of striping info from binary eg. `debuginfo`

#### debug-assertions
Controls if `debug_asset!` macro is compiled in or out

#### overflow-checks
Controls if runtime integer overflow checks are enabled, thus panic will occur.

#### lto
Configures link time optimization level

#### panic
Controls if panic should unwind stack or hard abort process upon panic.

#### incremental
Disables or enables incremental compilation.

#### codegen-units
Controls how many code generation units the crate will be split to.

#### rpath
Control if rpath is enabled or not.

## Creating crate on crates.io

1. Set-up profile and retrieve api-key
2. `cargo login` api-key (::secret::)
3. update crate metadata in Cargo.toml
   - add `name = "my_crate_name"` under `[package]`
   - add `license = "your selected license"` see [here](https://spdx.org/licenses/)
     - or license-file if different that SPDX one
   - `version`
   - `edition`
   - `desription`
   - generally everything needed is describe [here](https://doc.rust-lang.org/cargo/)
4. `cargo publish` 
   
## Updating a create on crates.io
1. Make changes
2. Update the version - good to keep semantic versioning rules, see [here](https://semver.org/)
3. `cargo publish`

## Deprecating old versions
`cargo yank --vers 1.0.1`

## Undeprecating old versions
`cargo yank --vers 1.0.1 --undo`

## Workspaces
Workspace is a set of pacgkages that shares same Cargo.lock (external packages versions) and output directory.

Creating workspace goes as follows:
1. create a directory
   - `mkdir my_workspace`
2. create a `Cargo.toml` file
   - `echo -e '[workspace]\nresolver = "3"' > Cargo.toml` setting resolver version withouth packages
3. create new binary using standard `cargo new <binary>`
   - you can create more binaries or libs in same way, they will be added to `members` section in main Cargo.toml
  
To have dependencies between packages inside workpsace solvable, paths need to be setup in given binaries Cargo.toml files like below:
```toml
...
[dependencies]
add_one = { path = "../add_one" }
...
```
To run specifgc binary from the workspace `cargo run -p <binary>`

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

## Comments
### Code comments
Use `// ` for commenting code

### Documentation comments
Use `/// ` for creating documentation comments that will be visible in HTML docs.
Markdown syntax is supported.

Documentation comments that have rust syntax embedded are automatically converted to tests.

### Contained items comment
Use `//! ` to comment module or a crate as a whole, not an item following as in the `/// `

## Borrows
  `&` - acts as a reference (borrows instance)

## Strings
  `str` - basic string slice type to which String is convertible

  `"\` - backslash after openiong double quote tells Rust not to put a newline character at the begining

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
        Some(format!("{state:?} is pretty old, for America"))
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

`'` is used to annotate generic lifetime relationship (lifetime name starts with it).

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


### Lifetime elision rules
Compiler uses three rules so reference parameters 
1. compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32.`
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.


### Static lifetime
`'static` denotes that the affected reference can live for the entire duration of the program.


## Testing

`cargo test` - runs all tests

Below attribute annotates that function is a test function
```rust
#[test]
```

### Useful macros
- `assert!(var: bool)`
- `assert_eq!(left: &T, right: &T)`
- `assert_ne!(left: &T, right: &T)`

`assert_eq` and `assert_ne` uses `==` and `!=` operators so compared types need to implement `PartialEq` trait (and `Debug` for printing). Normally adding `#[derive(PartialEq, Debug)]` to custom struct should be enough.

    Asserts can print custom messages (add as a last argument, can use `!format` strings)
### Panics
`#[should_panic]` attribute checks if tests panics (as an expectation)

`#[should_panic(expected = "expected panic text")]` - make panic check more precise, panic text need to contain a **expected substring**.

### Result<T, E> in tests

Tests can return Result<T, E> type sa an return value to conveniently fail. See below:

```rust
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
```
`Ok` means pass, `Err` fail. **`#[should_panic]` can't be used with such tests**


### Running tests
#### Basic cargo usage
`cargo test` to run tests

` -- ` separator is used to pass parameters to test binary instead of `cargo test`

`cargo test --help` vs `cargo test -- --help`

`cargo test -- --test-threads=1` to select how much threads to allocate for tests running

`cargo test -- --show-output` to show output even if tests are passing

#### Running tests by name

`cargo test <test_namesubstring>` - run tests matching given substring

#### Ignoring tests

Use ignore atribute to ignore test execution
```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

> [!important]
> `ignore` attrubute need to be after `test` one

To run ignoredd tests use `cargo test -- --ignored` (params to test binary not cargo)

To run all test including ignored ones
`cargo test -- --include-ignored`


### Testing convention - Unit tests

Unit tests should be put in src directory in each file with the code that they are testing. Tests should be in `test` module and be annotated with `cfg(test)`

The `#[cfg(test)]` annotation tells rust compiler to build and run these only in `cargo test` targe (in rust it is called configuration option). (it a bit like #ifdef X for conditional builds in cpp)

### Testing private functions

Whenever it is or not a good practice to do it, in Rust private (not `pub`) functions can be tested, as tests module is a child of parent module.

```rust
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```
as scope of parent is bring in by `use super::*`
test can call parent functions (even if these are not "exported")

### Testing convention - Integration tests
For integration tests the story is a bit different. Integration tests are using only public interface of a library.

Integration tests are put in `tests` directory.

There can be multiple files with tests. Cargo will compile all of them to separate crates.

`cargo test --test integration_test` to run `integration_test` suite (from that crate)

#### Sharing common code between tests

Instead of creating `tests/common.rs` file with common methods (it will produce `Running tests/common.rs` section without any tests)

You should use old mod convention
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
so create `common` directory with `mod.rs` file inside

> [!important]
> Files in subdirectories of `tests` directory are not compiled as separate crates

##### Integration Tests for Binary Crates
As binary create does not expose public functions this is not possible.

Common approach is to have both simple `src/main.rs` and implementation as `src/lib.rs` which exposes interface.


## Program arguments

`std::env::args()` provide and interator to access command line arguments of a program. To get a collection from that iterator, we need to call an `.collect()` method.

> [!important]
> If unicode character support is needed, instead of `args`, `args_os` need to be used.
>

## File System

Reading file can be done using `std::fs::read_to_string(file_path)`.

## Process exit code

Use `std::process::exit(x)` for that

## Environment variables

Use `use std::env::var("FLAG").is_ok()` to to check for existence **(not value)**

## Standard error output stream

Use `eprintln!` macro instead of `println!`.


## Closures
A bit simmilar to lambdas (but not quite). Can capture context from a scope. 

#### Types

- `FnOnce()` - closures that can be called once. Moves captures values out of its body.
- `FnMut()` - don't move captured values out of their body but can mutate captured values.
- `Fn()` - don't move captured values out of their body and dont mutate captured values.


#### Examples

Closure that calls a method on object
```rust
// snip

user_preference.unwrap_or_else(|| self.most_stocked())

// snip
```

Closure that sleeps (got parameter and explicitly declares a return type)

```rust
// snip
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
// snip
```

### Comparision to functions syntax
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

### Closure that takes an ownership of the value

Use `move` keyword before `||`

example:
```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

## Iterators
Used as any other languages that implements iterators, iterates over a set of values.

All iterators implements a `Iterator` trait.

**Consuming adapters** are iterator methods that consume iterator (like `iter().sum()`)

**Iterator adapters** does not consume iterator, they produce different iterators by changng some aspect of an original (like `iter().map(F)`).


### Some iterators methods
- `map`
- `sum`
- `filter`
- `collect`


## Public reexports
You can use `pub use` to reexport items.

``` rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;      // this is re-export as art::PrimaryColor
pub use self::kinds::SecondaryColor;    // this is re-export as art::SecondaryColor
pub use self::utils::mix;               // this is re-export as art::mix

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}

```

## Smart pointers

Pointer works simmilary as in C++. Reference `&` is a pointer that borrow value its points to.
References does not have any special capabilites nor overhead. That is not true for smart pointers.

Smart pointers are usually implemented using structs. They implement `Deref` and `Drop` traits. `Deref` allows an instance of smart pointer ot behave like a reference. `Drop` allows to customize destruction (going out of scope).

### Common smart pointers
 - `Box<T>`, for allocating values on a heap. Simmilar to `unique_ptr<T>` in C++.
 - `Rc<T>`, a reference counting type that enables multiple ownership. Something like `shared_ptr<T>` in C++.
 - `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>`, a type that enforces borrowing rules at runtime instead of compile time. I don't think we have something simmilar in C++ as C++ does not have borrow checker.
    - `RefCell<T>` is a runtime borrow checker for interior mutability (refcell is immutable but contents can be mutated by borrowing - `borrow_mut()`)

### Deref

To enable `*` operator on your type implement `Deref` trait (`deref` method). Under the hood rust will `*(instance.deref())` when calling `*`.

`Deref` coercion converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns &str. 

#### Mutability and deref coercion

Rust does deref coercion when it finds types and trait implementations in three cases:
- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

### Drop
This trait works like destructor in C++. Implements `drop` method which can't be called manually.
To release resources manually `std::mem::drop(instance)` can be used. 
