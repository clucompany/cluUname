
use crate::element::UtsElement;
use std::ops::Deref;
use std::fmt::Display;
use std::fmt;


#[derive(Debug)]
pub struct DisplayUts<T>(T);

impl<T: UtsElement> DisplayUts<T> {
	#[inline]
	pub fn new(a: T) -> Self {
		DisplayUts(a)
	}
}

impl<'a, T: 'a + UtsElement> DisplayUts<T> {
	#[inline(always)]
	pub fn as_element(&self) -> &(dyn UtsElement + 'a) {
		&self.0
	}
}

impl<T> Deref for DisplayUts<T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: UtsElement> From<T> for DisplayUts<T> {
	#[inline(always)]
	fn from(t: T) -> Self {
		Self::new(t)
	}
}

impl<T: UtsElement> AsRef<[u8]> for DisplayUts<T> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_array()
	}
}

impl<T> AsRef<T> for DisplayUts<T> {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.0	
	}
}

/*impl<'a, T: UtsElement> Display for DisplayUts<&'a T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		//CLONE CSTR DEBUG!
		let array = self.0.as_array();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
}*/

impl<T: UtsElement> Display for DisplayUts<T> {
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.display_fmt(fmt)
	}
}
