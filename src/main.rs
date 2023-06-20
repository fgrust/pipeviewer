use clap::Parser;
use pipeviewer::{args::Cli, read, stats, write};

use std::env;
use std::io::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let infile = cli.infile.unwrap_or_default();
    let outfile = cli.outfile.unwrap_or_default();
    let silent = cli.silent || !env::var_os("PV_SILENT").unwrap_or_default().is_empty();

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have 'crashed'
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // returns an error if any threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
