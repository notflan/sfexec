use sha2::{Sha256, Digest};

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn hash_iter_okay()
    {
	let hash_this = vec![vec![0,1,2,3,4],
			     vec![5,6,7,8,9],
			     vec![10,11,12,13,14],
			     vec![15,16,17,18,19]];

	let mut digest = Sha256::new();
	let mut digest2 = Sha256::new();
	for byte in hash_this.into_iter().into_hash_iter(&mut digest)
	{
	    digest2.input(&byte[..]);
	}

	assert_eq!(digest.result()[..], digest2.result()[..]);
    }

    #[test]
    fn copy_slice_works()
    {
	let slice = [0xab, 0xad, 0xca, 0xfe];
	let mut output = [0u8; 4];

	assert_eq!(copy_slice(&mut output, &slice), 4);
	assert_eq!(slice[..], output[..]);
    }
}

pub struct HashingIter<'a, I, T>
where I: Iterator<Item=T>,
      T: AsRef<[u8]>
{
    iter: I,
    digest: &'a mut Sha256,
}


impl<'a, I, T> Iterator for HashingIter<'a, I, T>
where I: Iterator<Item=T>,
      T: AsRef<[u8]>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
	match self.iter.next() {
	    Some(value) => {
		self.digest.input(value.as_ref());
		Some(value)
	    },
	    None => None,
	}
    }
}

pub trait HashingIterExt: Iterator + Sized
where <Self as Iterator>::Item: AsRef<[u8]>
{
    fn into_hash_iter<'a>(self, hash: &'a mut Sha256) -> HashingIter<'a, Self, <Self as Iterator>::Item>;
}

impl<I, T> HashingIterExt for I
where I: Iterator<Item=T>,
      T: AsRef<[u8]>
{
    fn into_hash_iter<'a>(self, hash: &'a mut Sha256) -> HashingIter<'a, I, T>
    {
	HashingIter {
	    iter: self,
	    digest: hash,
	}
    }
}

pub fn copy_slice<T>(mut dst: impl AsMut<[T]>, src: impl AsRef<[T]>) -> usize
    where T: Clone
{
    let dst = dst.as_mut();
    let src = src.as_ref();

    let mut i=0;
    for (d, s) in dst.iter_mut().zip(src.iter())
    {
	*d = s.clone();
	i+=1;
    }

    i
}
