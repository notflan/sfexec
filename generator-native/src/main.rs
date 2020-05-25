#![allow(dead_code)]

extern crate heaparray;

use std::{
    fs::{
	self, File, OpenOptions,
    },
    io,
    error::Error,
};

mod iter;
use iter::prelude::*;

mod translate;

mod opt;
mod arg;

fn usage() -> ! {
    let prog = &arg::program_name();
    println!("Usage: {} [-s] [-e <exec string>] [-o <output file>] <files...>", prog);
    println!("Usage: {} -h", prog);
    println!();
    println!(" -h\t\tPrint this message.");
    println!(" -s\t\tSilent mode.");
    println!(" -e <exec>\tScript to run after extraction.");
    println!(" -o <file>\tOutput filename.");
    std::process::exit(1);
}

fn main() -> Result<(), Box<dyn Error>>{

    /*let file = OpenOptions::new()
	.read(true)
	.open("test.txt")?;


    println!("{{");
    for buf in file.into_iter(4)
	.map(|byte| format!("0x{:02x},", byte))
	.group_at(4)
	.map(|strs| format!("\t{}", strs.join(" ")))
    {
	println!("{}", buf);
    }
    println!("}}");*/

    match arg::parse()? {
	arg::OperationMode::Normal(options, files) => {
	    //TODO: Operations
	},
	arg::OperationMode::Help => {
	    usage();
	},
    };

    Ok(())
}
