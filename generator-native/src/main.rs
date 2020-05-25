#![allow(dead_code)]

extern crate heaparray;

use std::{
    fs::{
	self, File, OpenOptions,
    },
    io::{self, Write,Read,},
    error::Error,
};

mod iter;
use iter::prelude::*;

mod translate;

#[macro_use]
mod opt;
mod arg;

use opt::Opt;

fn usage() -> ! {
    let prog = &arg::program_name();
    println!("Usage: {} [-s] [-e <exec string>] [-o <output file>] [-] <files...>", prog);
    println!("Usage: {} -h", prog);
    println!();
    println!(" -\t\tStop reading options.");
    println!(" -h\t\tPrint this message.");
    println!(" -s\t\tSilent mode.");
    println!(" -e <exec>\tScript to run after extraction.");
    println!(" -o <file>\tOutput filename.");
    std::process::exit(1)
}

fn write_file<From, To>(from: From, to: &mut To) -> io::Result<usize>
where From: Read,
      To: Write + ?Sized
{
    let mut count =0;
    for buf in from.into_iter(8)
	.map(|byte| {
	    count += 1;
	    byte
	})
	.map(|byte| format!("0x{:02x},", byte))
	.group_at(8)
	.map(|strs| format!("\t{}", strs.join(" ")))
    {
	println!("{}", buf);
    }

    Ok(count)
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
	    let output = options.find(|opt| match opt {
		Opt::Output(output) => Some(output.as_str()),
		_ => None
	    }).unwrap_or("file.h");
	    let exec = options.find(|opt| match opt {
		Opt::Execute(exec) => Some(exec.as_str()),
		_ => None
	    });
	    let silent = options.has_tag(&Opt::Silent);

	    println!("Writing to {}...", output);
	    let mut fp = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(output)?;

	    if silent {
		writeln!(fp, "#define SILENT")?;
	    }

	    writeln!(fp, "constexpr const int DATA_COUNT = {};", files.len())?;

	    if let Some(exec) = exec {
		let exec = translate::c_escape(exec);
		writeln!(fp, "constexpr const char* const DATA_EXEC_AFTER = {};", exec)?;
		writeln!(fp, "static constexpr auto DATA_EXEC_AFTER_HASH = {}_sha256;", exec)?;
	    } else {
		writeln!(fp, "constexpr const char* const DATA_EXEC_AFTER = nullptr;")?;
		writeln!(fp, "static constexpr auto DATA_EXEC_AFTER_HASH = \"unbound\"_sha256;")?;
	    }

	    let mut sizes = Vec::with_capacity(files.len());
	    writeln!(fp, "constexpr const unsigned char DATA[] = {{")?;

	    for file in files.iter() {
		let rfp = OpenOptions::new()
		    .read(true)
		    .open(file)?;
		print!(" + {}", file);

		sizes.push(write_file(rfp, &mut fp)?);
	    }
	},
	arg::OperationMode::Help => {
	    usage();
	},
    };

    Ok(())
}
