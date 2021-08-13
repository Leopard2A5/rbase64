extern crate clap;
extern crate rbase64;

use std::io;
use std::io::Write;

use clap::Clap;

fn main() -> Result<(), std::io::Error> {
    let args: Rbase64 = Rbase64::parse();

    let result = match args.decode {
        true => rbase64::decode(io::stdin()),
        false => rbase64::encode(io::stdin()),
    }?;

    io::stdout().write_all(&result)?;
    Ok(())
}

#[derive(Clap, Debug)]
#[clap(name = "rbase64", about = "encodes or decodes base64")]
struct Rbase64 {
    #[clap(short('D'), long)]
    decode: bool,
}
