
/**
 * By BingLi224
 * 12:59 THA 07/11/2019
 *
 * Simple test of include!(), over "the name `...` is defined multiple times" error
 */

// import instead of a.rs and b.rs
use std::convert::From;

include ! ( "a.rs" );
include ! ( "b.rs" );

fn main() {
    let a = A::from ( 7 );
    let b = B::from ( 11 );
    println!( "A={:?}, B={:?}", a, b );
}
