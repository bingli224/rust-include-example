
// imported in main.rs
//use std::convert::From;

#[derive(Debug)]
pub struct B { pub v : i32 }

impl From<i32> for B {
    fn from ( i: i32 ) -> Self {
        B { v: i }
    }
}

