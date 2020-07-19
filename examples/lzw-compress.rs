//! Compresses the input from stdin and writes the result to stdout.

use std::io::{self, BufWriter};

fn main() {
    match (|| -> io::Result<()> {
        let mut encoder = weezl::enlzw::Encoder::new(weezl::ByteOrder::Msb, 8);
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let stdout = io::stdout();
        let stdout = BufWriter::new(stdout.lock());
        encoder.encode_all(stdin, stdout).status?;
        Ok(())
    })() {
        Ok(()) => (),
        Err(err) => eprintln!("{}", err),
    }
    
}
