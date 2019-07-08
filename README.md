# comp_input

[crates.io](https://crates.io/crates/comp_input)
[docs.rs](https://docs.rs/comp_input/)

Input library for competitive programming.

## Example: weighted graph as edge list
```rust,no_run
#[macro_use]
extern crate comp_input;

fn main() {
    input! {
        n, m: usize,
        edges: [(usize1, usize1, u64); m],
    }
}
```

The variables `n`, `m` and `edges` then exist as local variables in scope.

# List of input fragments

| Fragment | Description |
|----------|-------------|
| `u8, u16, u32, u64, usize` | Unsigned integer (base 10) |
| `i8, i16, i32, i64, isize` | Signed integer (base 10, optional +/- prefix) |
| `usize1` | Like `usize`, but subtract 1 from the result (useful for 1-based input formats) |
| `char` | A single character |
| `String` | A sequence of non-ASCII-whitespace characters |
| `(T1, T2), (T1, T2, T3), ...` | Heterogeneous tuple of other input fragments, read in order |
| `[<T>; <n: expr>]` | `n` items parsed against `T`, returned as `Vec` |
| `[<T>; const <n>]` | `n` items parsed against `T`, in an array. `n` must be compile-time constant |

# TODO

 - Allow arbitrary parse functions
 - Branching
 - Loops (e.g. parse until -1)

# License

MIT/Apache-2.0
