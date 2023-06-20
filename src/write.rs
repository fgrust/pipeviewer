use crossbeam::channel::Receiver;

use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::path::Path;

pub fn write_loop(outfile: &Path, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.as_os_str().is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        let buf = write_rx.recv().unwrap();
        if buf.is_empty() {
            break;
        }
        if let Err(e) = writer.write_all(&buf) {
            if e.kind() == ErrorKind::BrokenPipe {
                // stop the program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
