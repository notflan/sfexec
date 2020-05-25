use std::{
    io::{
	Read,
    },
};
use heaparray::{
    heap,
};

pub struct BufferedReadIter<F>
where F: Read
{
    iter: F,
    buffer: Box<[u8]>,
    buffer_len: usize,
}

impl<F: Read> Iterator for BufferedReadIter<F>
{
    type Item = Box<[u8]>;

    fn next(&mut self) -> Option<Self::Item>
    {
	if self.buffer_len >= self.buffer.len() {
	    // Buffer is full.
	    return Some(self.swap_out());
	}
	
	if let Ok(read) = self.iter.read(&mut self.buffer[self.buffer_len..])
	{
	    self.buffer_len += read;
	    if self.buffer_len ==0 {
		None
	    } else {
		Some(self.swap_out())
	    }
	} else {
	    None
	}
    }
}

impl<F: Read> BufferedReadIter<F>
{
    fn swap_out(&mut self) -> Box<[u8]>
    {
	let len = self.buffer_len;
	self.buffer_len=0;

	heaparray::box_slice(&mut self.buffer[..len])
    }

    pub fn new(iter: F, buffer_len: usize) -> Self {
	let buffer = heap![u8; buffer_len].into_box();
	Self {
	    iter,
	    buffer_len: 0,
	    buffer,
	}
    }
}

pub trait ReadIterExt: Read + Sized
{
    fn into_iter(self, buffer_len: usize) -> BufferedReadIter<Self>
    {
	BufferedReadIter::new(self, buffer_len)
    }
}
impl<T: Read> ReadIterExt for T{}

