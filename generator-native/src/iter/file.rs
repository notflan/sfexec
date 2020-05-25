use std::{
    io::{
	Read,
    },
};

pub struct ByteIter<F: Read>
    (F, [u8; 1]);

impl<T> Iterator for ByteIter<T>
where T: Read
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>
    {
	if let Ok(read) = self.0.read(&mut self.1[..])
	{
	    if read < 1 {
		None
	    } else {
		Some(self.1[0])
	    }
	} else {
	    None
	}
    }
}

impl<T> ByteIter<T>
where T: Read
{
    fn new(stream: T) -> Self {
	Self(stream, [0u8; 1])
    }
}

pub trait ByteIterExt: Read + Sized
{
    fn into_byte_iter(self) -> ByteIter<Self>
    {
	ByteIter::new(self)
    }
}
impl<T: Read> ByteIterExt for T{}
