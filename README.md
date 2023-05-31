# Default2

[![Test](https://github.com/yaa110/default2/actions/workflows/build.yml/badge.svg)](https://github.com/yaa110/default2/actions/workflows/build.yml) [![crates.io](https://img.shields.io/crates/v/default2.svg)](https://crates.io/crates/default2)

Default implementation using macros

## Example

Use `Default2` to set default value of each field using a macro:

```rust
use default2::Default2;

#[derive(Default2)]
struct Process {
    #[default(10)]
    id: i32,
    #[default("main".into())]
    name: String,
    #[default(num_cpus::get())]
    cpus: usize,
    #[default(vec![1, 2, 3])]
    vector: Vec<u64>,
    payload: u64,
}
```

The following code will be generated:

```rust
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
            vector: vec![1, 2, 3],
            payload: Default::default(),
        }
    }
}
```
