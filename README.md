# Default Macro

[![Test](https://github.com/yaa110/default2/actions/workflows/build.yml/badge.svg)](https://github.com/yaa110/default2/actions/workflows/build.yml) [![crates.io](https://img.shields.io/crates/v/default2.svg)](https://crates.io/crates/default2)

A convenient macro to implement `Default` for structs using field initializers.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
default2 = "2"
```

## Example

Use the `default2::default!` macro to define a struct and its default values in one place.

```rust
default2::default! {
    pub struct Process {
        id: i32 = 10,
        name: String = "main".into(),
        cpus: usize = num_cpus::get(),
        payload: u64,
    }
}
```

The macro will generate the standard struct definition along with a `Default` implementation:

```rust,ignore
pub struct Process {
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

### Const Defaults

You can also generate a `const` default function by adding the `#[const_default]` attribute to your struct. This will generate an inherent method `const_default()`.

You are responsible for ensuring that all default value expressions are valid in a `const` context.

```rust,ignore
use default2::default;

default! {
    #[const_default]
    pub struct MyConfig {
        timeout: u32 = 500,
        name: &'static str = "default_config",
    }
}

// You can now create a const instance:
const MY_CONFIG: MyConfig = MyConfig::const_default();
assert_eq!(MY_CONFIG.timeout, 500);

// The standard non-const Default trait is still implemented:
let other_config = MyConfig::default();
assert_eq!(other_config.name, "default_config");
```
