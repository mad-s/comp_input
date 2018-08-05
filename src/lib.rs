//! Input library for competitive programming.
//!
//! Built for speed and to be callable in a concise manner, so no
//! good error handling (just panics) or thread safety.
//!
//! You need to call `init()` before trying to read anything, this
//! sets up the global stdin lock used.
//!
//! # Example
//! ```rust,no_run
//! extern crate comp_input;
//! use comp_input::input;
//!
//! fn main() {
//!     comp_input::init();
//!
//!     let line1 : String = input();
//!     let line2 : Vec<u8> = input();
//!     let line3 : (char, f64) = input();
//!     println!("{:?} {:?} {:?}", line1, line2, line3);
//! }
//! ```

use std::str::FromStr;

static mut GLOBAL_STDIN : Option<std::io::Stdin>     = None;
static mut GLOBAL_LOCK  : Option<std::io::StdinLock> = None;

/// The input trait allows for being read from standard input, usually consuming one line.
///
/// For performance reasons, a global lock on Stdin is used, so you
/// can only use this after calling `init()` and only on a single
/// thread.
pub trait Input : Sized {
    /// Read one instance of `Self` from standard input
    fn input() -> Self;
}

/// Read a line, not including a newline character
impl Input for String {
    fn input() -> String {
        let mut line = String::new();
        use std::io::BufRead;
        unsafe {
            GLOBAL_LOCK.as_mut().unwrap().read_line(&mut line).unwrap();
        }
        if line.ends_with('\n') {
            line.pop();
        }
        line
    }
}

/// Discard a line
impl Input for () {
    fn input() -> Self {
        drop(String::input());
    }
}

macro_rules! primitive_impls {
    ($($T:ty)*) => {
        $(
            impl Input for $T {
                fn input() -> Self {
                    let line = String::input();
                    Self::from_str(line.trim_right()).unwrap()
                }
            }
        )*
    }
}
primitive_impls! {
    u8 i8
    u16 i16
    u32 i32
    u64 i64
    usize isize
    f32 f64
    char
}

/// Input is implemented for fixed-sized arrays and `Vec`'s of types
/// implementing FromStr. The input line is split on whitespace and
/// each word is parsed seperately.
impl<T: FromStr> Input for Vec<T> {
    fn input() -> Self {
        let line = String::input();
        line.split_whitespace()
            .map(|word| T::from_str(word).ok().unwrap())
            .collect::<Vec<T>>()
    }
}

macro_rules! array_impls {
    ($len:expr; $($idx:expr)*) => {
        impl <T: FromStr> Input for [T; $len] {
            fn input() -> Self {
                let line = String::input();
                let words = line.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(words.len(), $len, "Expected {} values but got {}", $len, words.len());
                [
                    $(T::from_str(words[$idx]).ok().unwrap()),*
                ]
            }
        }
    }
}

array_impls!(2;  0 1);
array_impls!(3;  0 1 2);
array_impls!(4;  0 1 2 3);
array_impls!(5;  0 1 2 3 4);
array_impls!(6;  0 1 2 3 4 5);
array_impls!(7;  0 1 2 3 4 5 6);
array_impls!(8;  0 1 2 3 4 5 6 7);
array_impls!(9;  0 1 2 3 4 5 6 7 8);
array_impls!(10; 0 1 2 3 4 5 6 7 8 9);
array_impls!(11; 0 1 2 3 4 5 6 7 8 9 10);
array_impls!(12; 0 1 2 3 4 5 6 7 8 9 10 11);
array_impls!(13; 0 1 2 3 4 5 6 7 8 9 10 11 12);
array_impls!(14; 0 1 2 3 4 5 6 7 8 9 10 11 12 13);
array_impls!(15; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
array_impls!(16; 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);

macro_rules! tuple_impls {
    ($len:expr; $($idx:tt $T:ident)*) => {
        impl<$($T : FromStr),+> Input for ($($T,)*) {
            fn input() -> Self {
                let line = String::input();
                let words = line.split_whitespace().collect::<Vec<&str>>();
                assert_eq!(words.len(), $len, "Expected {} values but got {}", $len, words.len());
                (
                    $($T::from_str(words[$idx]).ok().unwrap(),)*
                )
            }
        }
    }
}

tuple_impls!(1;  0 T0);
tuple_impls!(2;  0 T0 1 T1);
tuple_impls!(3;  0 T0 1 T1 2 T2);
tuple_impls!(4;  0 T0 1 T1 2 T2 3 T3);
tuple_impls!(5;  0 T0 1 T1 2 T2 3 T3 4 T4);
tuple_impls!(6;  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
tuple_impls!(7;  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
tuple_impls!(8;  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
tuple_impls!(9;  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
tuple_impls!(10; 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);




/// Initialize the global stdin lock, required before calling `input()`.
pub fn init() {
    unsafe {
        GLOBAL_STDIN = Some(std::io::stdin());
        GLOBAL_LOCK  = Some(GLOBAL_STDIN.as_ref().unwrap().lock());
    }
}

/// Read an instance of T from the next line of stdin.
///
/// A call to `init()` is required before using this.
pub fn input<T: Input>() -> T {
    T::input()
}
