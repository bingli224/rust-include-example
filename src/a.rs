
// imported in main.rs
//use std::convert::From;

#[derive(Debug)]
pub struct A { pub v : i32 }

impl From<i32> for A {
    fn from ( i: i32 ) -> Self {
        A { v: i }
    }
}
