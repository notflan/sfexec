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

fn main() -> Result<(), Box<dyn Error>>{

    /*let file = OpenOptions::new()
	.read(true)
	.open("test.txt")?;

    for buf in file.into_iter(2).group_at(2)
    {
	println!("{:?}", buf);
}*/

    

    Ok(())
}
