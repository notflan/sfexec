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

fn main() -> Result<(), Box<dyn Error>>{

    let file = OpenOptions::new()
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
    println!("}}");

    Ok(())
}
