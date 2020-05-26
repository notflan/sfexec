use std::{
    io::{
	Read,
    },
};

pub struct BufferedReadIter<F>
where F: Read
{
    iter: F,
    buffer: Box<[u8]>,
    buffer_len: usize,
    buffer_push: usize,
}

impl<F: Read> Iterator for BufferedReadIter<F>
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
	if self.buffer_len > 0 {
	    let output = self.buffer[self.buffer_push];
	    self.buffer_push += 1;
	    self.buffer_len -= 1;

	    return Some(output);
	}
	self.buffer_push = 0;

	
	if let Ok(read) = self.iter.read(&mut self.buffer[self.buffer_len..])
	{
	    self.buffer_len += read;
	    if self.buffer_len == 0 {
		None
	    } else {
		self.next()
	    }
	} else {
	    None
	}
    }
}

impl<F: Read> BufferedReadIter<F>
{
    pub fn new(iter: F, buffer_len: usize) -> Self {
	let buffer = vec![0u8; buffer_len].into_boxed_slice();
	Self {
	    iter,
	    buffer_len: 0,
	    buffer_push: 0,
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

