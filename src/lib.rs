//! Input library for competitive programming.
//!
//! # Example: reading a graph given as an edge list
//! ```rust,no_run
//! #[macro_use]
//! extern crate comp_input;
//!
//! fn main() {
//!     input! {
//!         n, m: usize,
//!         edges: [(usize1, usize1); m]
//!     }
//! }
//! ```

use std::str::FromStr;
use std::io::BufRead;

extern crate memchr;
use memchr::{memchr};

trait BufReadExt : BufRead {
    #[inline]
    fn fill_buf_nonempty(&mut self) -> ::std::io::Result<&[u8]> {
        let buf = self.fill_buf()?;
        if !buf.is_empty() {
            Ok(buf)
        } else {
            Err(::std::io::ErrorKind::UnexpectedEof.into())
        }
    }
}

impl<R: BufRead> BufReadExt for R {}

pub trait FromAscii : Sized {
    fn from_ascii(src: &[u8]) -> Option<Self>;
}

macro_rules! from_ascii_int_impl {
    ($($t:ty)*) => {
        $(
            impl FromAscii for $t {
                #[inline]
                fn from_ascii(src: &[u8]) -> Option<$t> {
                    if src.is_empty() {
                        return None
                    }

                    let (sign, digits) = if src[0] == b'+' || src[0] == b'-' {
                        if src.len() == 1 {
                            return None
                        }
                        (src[0] == b'-', &src[1..])
                    } else {
                        (false, src)
                    };

                    let mut res : $t = 0;
                    for &c in digits {
                        let x = (c as char).to_digit(10)? as $t;
                        res = res.wrapping_mul(10).wrapping_add(x);
                    }

                    if sign {
                        Some(-res)
                    } else {
                        Some(res)
                    }
                }
            }
        )*
    }
}

macro_rules! from_ascii_uint_impl {
    ($($t:ty)*) => {
        $(
            impl FromAscii for $t {
                #[inline]
                fn from_ascii(src: &[u8]) -> Option<$t> {
                    if src.is_empty() {
                        return None
                    }

                    let digits = src;

                    let mut res : $t = 0;
                    for &c in digits {
                        let x = (c as char).to_digit(10)? as $t;
                        res = res.wrapping_mul(10).wrapping_add(x);
                    }

                    Some(res)
                }
            }
        )*
    }
}
from_ascii_uint_impl! { u8 u16 u32 u64 usize }
from_ascii_int_impl!  { i8 i16 i32 i64 isize }

impl FromAscii for char {
    #[inline]
    fn from_ascii(src: &[u8]) -> Option<char> {
        if src.len() != 1 {
            return None
        }
        Some(src[0] as char)
    }
}

impl FromAscii for String {
    #[inline]
    fn from_ascii(src: &[u8]) -> Option<String> {
        Some(std::str::from_utf8(src).ok()?.to_owned())
    }
}


pub struct FormattedRead<R: BufRead> {
    r: R,
    buf: Vec<u8>,
}

fn consume_ws<R: BufRead>(r: &mut R) -> std::io::Result<()> {
    loop {
        let buf = r.fill_buf_nonempty()?;
        if let Some(ix) = buf.iter().position(|&c| !c.is_ascii_whitespace()) {
            r.consume(ix);
            return Ok(());
        } else {
            let consume = buf.len();
            r.consume(consume);
        }
    }
}

impl<R: BufRead> FormattedRead<R> {
    pub fn new(r: R) -> Self {
        FormattedRead {
            r,
            buf: vec![]
        }
    }

    pub fn read_word<T: FromAscii>(&mut self) -> std::io::Result<T> {
        consume_ws(&mut self.r)?;
        let buf = self.r.fill_buf_nonempty()?;
        let split_ix = buf.iter().position(u8::is_ascii_whitespace);
        if let Some(ix) = split_ix {
            let res = T::from_ascii(&buf[..ix]).ok_or(std::io::ErrorKind::InvalidData)?;
            self.r.consume(ix+1);
            return Ok(res);
        }

        self.buf.clear();
        self.buf.extend_from_slice(buf);
        let l = buf.len();
        self.r.consume(l);

        loop {
            let buf = self.r.fill_buf_nonempty()?;
            if let Some(ix) = buf.iter().position(u8::is_ascii_whitespace) {
                self.buf.extend_from_slice(&buf[..ix]);
                let res = T::from_ascii(&self.buf).ok_or(std::io::ErrorKind::InvalidData)?;
                self.r.consume(ix+1); // maybe more?
                return Ok(res);
            } else {
                self.buf.extend_from_slice(&buf);
                let l = buf.len();
                self.r.consume(l);
            }
        }
    }

    pub fn read_line<T: FromStr>(&mut self) -> std::io::Result<T> {
        consume_ws(&mut self.r)?;
        let buf = self.r.fill_buf_nonempty()?;
        if let Some(ix) = memchr(b'\n', buf) {
            // CR-LF
            let split = ix.checked_sub(1).filter(|&i| buf[i] == b'\r').unwrap_or(ix);
            let res = std::str::from_utf8(&buf[..split]).map_err(|_| std::io::ErrorKind::InvalidData)?;
            let res = res.parse().map_err(|_| std::io::ErrorKind::InvalidData)?;
            self.r.consume(ix+1); // maybe more?
            return Ok(res);
        }
        self.buf.clear();
        self.buf.extend_from_slice(buf);
        let l = buf.len();
        self.r.consume(l);

        loop {
            let buf = self.r.fill_buf_nonempty()?;
            if let Some(ix) = memchr(b'\n', buf) {
                self.buf.extend_from_slice(&buf[..ix]);
                if self.buf[self.buf.len()-1] == b'\r' {
                    self.buf.pop();
                }

                let res = std::str::from_utf8(&self.buf).map_err(|_| std::io::ErrorKind::InvalidData)?;
                let res = res.parse().map_err(|_| std::io::ErrorKind::InvalidData)?;
                self.r.consume(ix+1); // maybe more?
                return Ok(res);
            } else {
                self.buf.extend_from_slice(&buf);
                let l = buf.len();
                self.r.consume(l);
            }
        }
    }
}

#[macro_export]
macro_rules! input {
    ($r:ident => $($($v:ident),* : $t:tt),*) => {
        $(
            $(
                let $v = read_one!($r => $t);
            )*
        )*
    };
    ($($($v:ident),* : $t:tt),*) => {
        let input__stdin = ::std::io::stdin();
        let mut input__reader = $crate::FormattedRead::new(input__stdin.lock());
        input!(input__reader => $($($v),* : $t),*);
        drop(input__reader);
    };
}

#[macro_export]
macro_rules! read_one {
    ($r:ident => [$t:tt; const $s:tt]) => {
        {
            let mut res = <[$t; $s]>::default();
            for i in 0..$s {
                res[i] = read_one!($r => $t);
            }
            res
        }
    };
    ($r:ident => [$t:tt; $s:tt]) => {
        (0..$s).map(|_| read_one!($r => $t)).collect::<Vec<_>>()
    };
    ($r:ident => ($($t:tt),*)) => {
        ($(
            read_one!($r => $t),
        )*)
    };
    ($r:ident => usize1) => {
        read_one!($r => usize) - 1
    };
    ($r:ident => {$r2:ident => $($t:tt)*}) => {
        {
            let $r2 = &mut $r;
            $($t)*
        }
    };
    ($r:ident => line) => {
        $r.read_line::<String>().expect("failed to read line")
    };
    ($r:ident => $t:ty) => {
        $r.read_word::<$t>().expect(concat!("failed to read ", stringify!($t)))
    };
}


#[test]
fn test_graph() {
    let input = b"3 4\n1 2\n1 3\n2 3\n2 1\n";
    let mut reader = FormattedRead::new(std::io::Cursor::new(&input[..]));

    input! {
        reader =>
            n, m: usize,
            edges: [(usize1, usize1); m]
    }

    assert_eq!(n, 3);
    assert_eq!(m, 4);
    assert_eq!(edges, vec![(0, 1), (0, 2), (1, 2), (1, 0)]);
}

#[test]
fn test_crlf() {
    let input = b"3 b\r\nHello World!\r\n-2 -1 0\r\nFino.\r\n";
    let mut reader = FormattedRead::new(std::io::Cursor::new(&input[..]));

    input! {
        reader =>
            a: u32,
            b: char,
            c: line,
            d: [i8; const 3],
            e: String
    }

    assert_eq!(a, 3);
    assert_eq!(b, 'b');
    assert_eq!(c, "Hello World!");
    assert_eq!(d, [-2, -1, 0]);
    assert_eq!(e, "Fino.");

}
