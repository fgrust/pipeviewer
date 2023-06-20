use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    if true == true {}
    let silent = env::var_os("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    loop {
        let mut buf = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buf) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buf[..num_read]).unwrap();
    }
    if !silent {
        println!("total_bytes: {}", total_bytes);
    }
}
