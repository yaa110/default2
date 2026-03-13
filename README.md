# Default Macro

[![Test](https://github.com/yaa110/default2/actions/workflows/build.yml/badge.svg)](https://github.com/yaa110/default2/actions/workflows/build.yml) [![crates.io](https://img.shields.io/crates/v/default2.svg)](https://crates.io/crates/default2)

A convenient macro to implement `Default` for structs using field initializers.

## Example

Add this to your `Cargo.toml`:

```toml
[dependencies]
default2 = "2"
```

Use the `default2::default!` macro to define a struct and its default values in one place.

```rust
default2::default! {
    #[derive(Debug, PartialEq)]
    struct Process {
        id: i32 = 10,
        name: String = "main".into(),
        cpus: usize = num_cpus::get(),
        payload: u64,
    }
}
```

The macro will generate the standard struct definition along with a `Default` implementation:

```rust
#[derive(Debug, PartialEq)]
struct Process {
    id: i32,
    name: String,
    cpus: usize,
    payload: u64,
}

impl Default for Process {
    fn default() -> Self {
        Process {
            id: 10,
            name: "main".into(),
            cpus: num_cpus::get(),
            payload: std::default::Default::default(),
        }
    }
}
```
