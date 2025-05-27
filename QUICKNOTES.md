

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
