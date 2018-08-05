# comp_input

[crates.io](https://crates.io/crates/comp_input)
[docs.rs](https://docs.rs/comp_input/)

Input library for competitive programming.

Built for speed and to be callable in a concise manner, so no
good error handling (just panics) or thread safety.

You need to call `init()` before trying to read anything, this
sets up the global stdin lock used.

## Example
```rust
extern crate comp_input;
use comp_input::input;

fn main() {
    comp_input::init();

    let line1 : String = input();
    let line2 : Vec<u8> = input();
    let line3 : (char, f64) = input();
    println!("{:?} {:?} {:?}", line1, line2, line3);
}
```

License: MIT/Apache-2.0
