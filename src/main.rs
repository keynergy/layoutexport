mod lib;
use std::{fs::File, io::Write};

use clap::{CommandFactory, ErrorKind, Parser};
use keynergy::Layout;

use crate::lib::XkbLayout;

/// Tool that exports a Keynergy layout.toml into Linux XKB
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to TOML file that contains the layout to export
    #[clap(short, long)]
    layout: String,
}

fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();
    let layout = match Layout::load(&args.layout) {
        Ok(l) => l,
        Err(_) => cmd
            .error(
                ErrorKind::Io,
                "Couldn't read that file. Make sure that it exists, and is a .toml layout file.",
            )
            .exit(),
    };
    let xkb_layout = match XkbLayout::from(&layout) {
        Ok(x) => x,
        Err(_) => panic!("Layout does not have a standard format!"),
    };

    let mut file = File::create(layout.name.replace(" ", "_").to_ascii_lowercase()).unwrap();
    match write!(file, "{}", xkb_layout.content) {
	Err(_) => panic!("Could not write the resulting file. Check permissions for this directory."),
	_ => 0
    };
}
