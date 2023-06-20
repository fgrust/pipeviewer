use crate::CHUNK_SIZE;

use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::path::Path;
use std::sync::mpsc::Sender;

pub fn read_loop(infile: &Path, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.as_os_str().is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buf = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        if stats_tx.send(Vec::from(&buf[..num_read])).is_err() {
            break;
        }
    }
    let _ = stats_tx.send(Vec::new());
    Ok(())
}
