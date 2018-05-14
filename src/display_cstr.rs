
/*!
Display trait for UtsName.
*/


use std::fmt::{self, Display};
use std::ffi::CStr;
use std::fmt::Write;

#[derive(Debug)]
pub struct DisplayCStr<'a> {
	cstr: &'a CStr,
}

impl<'a> DisplayCStr<'a> {
	#[inline]
	pub fn new(cstr: &'a CStr) -> DisplayCStr<'a> {
		DisplayCStr {
			cstr: cstr
		}
	}
}

impl<'a> Display for DisplayCStr<'a> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		//CLONE CSTR DEBUG!
		for byte in self.cstr.to_bytes().iter().flat_map(|&b| ::std::ascii::escape_default(b)) {
			fmt.write_char(byte as char)?;
		}
		
		Ok( () )
	}
}

