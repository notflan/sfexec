use std::{
    collections::HashSet,
    iter::FromIterator,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Opt
{
    Silent,
    Execute(String),
    Output(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options(HashSet<Opt>);

impl Options
{
    pub fn new() -> Self
    {
	Self(HashSet::new())
    }
    pub fn find<'a, T,F>(&'a self, pred: F) -> Option<T>
    where F: Fn(&'a Opt) -> Option<T>
    {
	for opt in self.0.iter() {
	    if let Some(ret) = pred(opt) {
		return Some(ret);
	    }
	}
	None
    }

    pub fn has_tag(&self, opt: &Opt) -> bool {
	for x in self.0.iter() {
	    if x == opt {
		return true;
	    }
	}
	false
    }
}

//operators

use std::ops;
// opts |= opt
impl ops::BitOrAssign<Opt> for Options
{
    fn bitor_assign(&mut self, rhs: Opt)
    {
	self.0.insert(rhs);
    }
}
// opts |= other_opts
impl ops::BitOrAssign for Options
{
    fn bitor_assign(&mut self, other: Self)
    {
	self.0.extend(other.0)
    }
}

// opt1 | opt2
impl ops::BitOr for Opt
{
    type Output = Options;

    fn bitor(self, other: Self) -> <Self as ops::BitOr>::Output
    {
	Options::from([self, other].to_vec())
    }
}

impl ops::BitAnd<Opt> for Options
{
    type Output = bool;

    fn bitand(self, other: Opt) -> bool
    {
	self.has_tag(&other)
    }
}
impl ops::BitAnd for Options
{
    type Output = bool;
    fn bitand(self, other: Self) -> bool
    {
	for x in other.0.iter()
	{
	    if !self.has_tag(x) {
		return false;
	    }
	}

	true
    }
}

impl From<Vec<Opt>> for Options
{
    fn from(other: Vec<Opt>) -> Self{
	let mut hs = HashSet::new();
	for x in other.into_iter()
	{
	    hs.insert(x);
	}
	Self(hs)
    }
}

impl From<Options> for Vec<Opt>
{
    fn from(other: Options) -> Vec<Opt>
    {
	Vec::from_iter(other.0.into_iter())
    }
}

impl IntoIterator for Options
{
    type Item = Opt;
    type IntoIter = std::vec::IntoIter<Opt>;

    fn into_iter(self) -> Self::IntoIter
    {
	Vec::from(self).into_iter()
    }
}
