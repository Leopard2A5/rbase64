extern crate rbase64;

use std::io;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let result = rbase64::encode(io::stdin())?;

    println!("result: {}", result);

    Ok(())
}
