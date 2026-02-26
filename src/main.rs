#![allow(
    unused_variables,
    dead_code,
    non_snake_case,
    unused_parens,
    unused_variables,
    non_snake_case,
    unreachable_code,
    unused_imports,
    unused_assignments
)]

mod internal;
use internal::Problems;

fn main() {
    let P = Problems;
    
    println!(" Number of Circular Primes {:?}",P.p35(1000000));
}
