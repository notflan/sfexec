#![allow(dead_code)]

#[cfg(feature = "hash")]
extern crate sha2;

use std::{
    fs::{
	OpenOptions,
    },
    io::{self, Write,Read,},
    error::Error,
    path::{Path,},
};

mod iter;
use iter::prelude::*;

mod translate;

#[macro_use]
mod opt;
mod arg;

use opt::Opt;

#[cfg(feature = "hash")]
mod hash;

macro_rules! flush {
    () => {
	std::io::stdout().flush().ok().expect("fatal: could not flush stdout")
    }
}

fn version(verbose: bool)
{
    if verbose {
	println!("sfexec-generator-native verison: {}", env!("CARGO_PKG_VERSION"));
	println!("author: {} (https://flanchan.moe/)", env!("CARGO_PKG_AUTHORS"));
	println!("license: GPL 3.0\n");
    } else {
	print!("{}", env!("CARGO_PKG_VERSION"));
    }
}

fn usage() -> ! {
    {
	let prog = &arg::program_name();
	version(true);
	println!("Usage: {} [-s] [-e <exec string>] [-o <output file>] [-u] [-] <files...>", prog);
	println!("Usage: {} -h", prog);
	println!("Usage: {} -v|-V", prog);
	println!();
	println!(" -h\t\tPrint this message.");
	println!(" -v\t\tPrint version.");
	println!(" -V\t\tPrint program info.");
	println!(" -\t\tStop reading options.");
	println!(" -s\t\tSilent mode.");
	println!(" -e <exec>\tScript to run after extraction.");
	println!(" -o <file>\tOutput filename.");
	#[cfg(feature = "hash")]
	println!(" -u\t\tUnchecked mode. Do not compute hashes of inputs.");
	#[cfg(not(feature = "hash"))]
	println!(" -u\t\tUnchecked mode. (default as generator is not compiled with `hash` feature).");
    }
    std::process::exit(1)
}

#[cfg(feature = "hash")]
type Sha256Hash = [u8; 32];
#[cfg(not(feature = "hash"))]
type Sha256Hash = ();

const WF_BUFFER_SIZE: usize = 1024;
const WF_GROUP_SIZE: usize = 16;

#[allow(unused_variables)]
fn write_file<From, To>(from: From, to: &mut To, hash: bool) -> io::Result<(Sha256Hash, usize)>
where From: Read,
      To: Write + ?Sized
{
    #[cfg(feature = "hash")]
    {
	use sha2::{Sha256,Digest};
	use hash::copy_slice;
	let mut hash_output = [0u8; 32];
	let mut count =0;
	let mut digest = Sha256::new();
	let lambda: Box<dyn FnMut(u8) -> String> = if hash {
	    Box::new(|byte| (digest.input(&[byte]), format!("0x{:02x},", byte)).1)
	} else {
	    Box::new(|byte| format!("0x{:02x},", byte))
	};
	for buf in from.into_iter(WF_BUFFER_SIZE)
	    .map(lambda)
	    .group_at(WF_GROUP_SIZE)
	    .map(|bytes| (count += bytes.len(), bytes).1)
	    .map(|strs| format!("\t{}", strs.join(" ")))
	{
	    writeln!(to, "{}", buf)?;
	}

	if hash {
	    copy_slice(&mut hash_output[..], &digest.result()[..]);
	}
	
	Ok((hash_output, count))
    }
    #[cfg(not(feature = "hash"))]
    {
	let mut count =0;
	for buf in from.into_iter(WF_BUFFER_SIZE)
	    .map(|byte| format!("0x{:02x},", byte))
	    .group_at(WF_GROUP_SIZE)
	    .map(|bytes| (count += bytes.len(), bytes).1)
	    .map(|strs| format!("\t{}", strs.join(" ")))
	{
	    writeln!(to, "{}", buf)?;
	}

	Ok(((), count))
    }
}

fn attempt_get_name<'a, P>(path: &'a P) -> Result<&'a str, &'static str>
where P: AsRef<Path> + ?Sized
{
    let path = path.as_ref();
    if let Some(path) = path.file_name() {
	if let Some(file_name) = path.to_str() {
	    Ok(file_name)
	}
	else {
	    Err("Invalid unicode in filename")
	}
    } else {
	Err("No filename, are you trying to add a directory?")
    }
}

#[allow(unused_variables)]
fn hash_str(hash: &Sha256Hash) -> String
{
    #[cfg(not(feature = "hash"))]
    return "0,".repeat(32);
    #[cfg(feature = "hash")]
    {
	let mut output = String::with_capacity(64);
	for byte in hash.iter().map(|byte| format!("0x{:02x}, ", *byte))
	{
	    output.push_str(&byte);
	}
	output
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
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
	    let no_hash = options.has_tag(&Opt::NoHash);

	    println!("Writing to {}...", output);
	    let mut fp = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(output)?;

	    if silent {
		writeln!(fp, "#define SILENT")?;
	    }

	    writeln!(fp, "constexpr const int DATA_COUNT = {};", files.len())?;

	    if let Some(exec) = exec {
		let exec = translate::c_escape(exec);
		writeln!(fp, "constexpr const char* const DATA_EXEC_AFTER = \"{}\";", exec)?;
		writeln!(fp, "static constexpr auto DATA_EXEC_AFTER_HASH = \"{}\"_sha256;", exec)?;
	    } else {
		writeln!(fp, "constexpr const char* const DATA_EXEC_AFTER = nullptr;")?;
		writeln!(fp, "static constexpr auto DATA_EXEC_AFTER_HASH = \"unbound\"_sha256;")?;
	    }

	    let mut sizes = Vec::with_capacity(files.len());
	    let mut hashes = Vec::with_capacity(files.len());
	    
	    writeln!(fp, "constexpr const unsigned char DATA[] = {{")?;

	    for file in files.iter() {
		print!(" + {}", file);
		flush!();
		let file = OpenOptions::new()
		    .read(true)
		    .open(file)?;

		sizes.push(match write_file(file, &mut fp, !no_hash) {
		    Ok((hash, size)) => {
			hashes.push(hash);
			println!(" OK");
			
			size
		    },
		    Err(error) => {
			println!(" FAILED: {}",error);
			return Err("state corrupted: cannot continue after mid-failed write.")?;
		    },
		});
	    }
	    writeln!(fp, "}};")?;
	    println!("Adding lengths...");

	    writeln!(fp, "constexpr const long DATA_LENGTHS[DATA_COUNT] = {{")?;
	    for size in sizes.into_iter() {
		write!(fp, "\t{}ll,", size)?;
		
	    }
	    writeln!(fp, "\n}};")?;

	    #[cfg(feature="hash")]
	    if !no_hash
	    {
		println!("Adding hashes...");
		writeln!(fp, "#define DATA_HASHED")?;
	    }
	    writeln!(fp, "constexpr const unsigned char DATA_HASHES[] = {{")?;
	    for hash in hashes.into_iter() {
		writeln!(fp, "\t{}", hash_str(&hash))?;
	    }
	    writeln!(fp, "}};")?;
	    
	    println!("Adding names...");

	    writeln!(fp, "constexpr const char* const DATA_NAMES[DATA_COUNT] = {{")?;
	    for file in files.into_iter() {
		let file = Path::new(&file);
		print!(" - {:?}", file);
		flush!();
		let file = match attempt_get_name(&file) {
		    Ok(file) => file,
		    Err(error) => {
			println!(" FAILED: pathspec: {}", error);

			return Err("name write failed, aborting.")?;
		    }
		};
		
		match writeln!(fp, "\t\"{}\",", translate::c_escape(file)) {
		    Err(error) => {
			println!(" FAILED: write: {}", error);

			return Err("name write failed, aborting.")?;
		    },
		    _ => (),
		};
		println!(" OK");
	    }
	    writeln!(fp, "}};")?;
	},
	arg::OperationMode::Help => {
	    usage();
	},
	arg::OperationMode::Version(verbose) => {
	    version(verbose);
	},
    };

    Ok(())
}
