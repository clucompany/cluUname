
/*!
Safe display of CStr
*/


use std::ffi::CString;
use std::fmt::{self, Display};
use std::ffi::CStr;
use std::fmt::Write;

#[derive(Debug)]
pub struct DisplayCStr<'a> {
	cstr: &'a CStr,
}

impl<'a> DisplayCStr<'a> {
	#[inline]
	pub fn new(cstr: &'a CStr) -> Self {
		Self {
			cstr: cstr
		}
	}
}

impl<'a> Display for DisplayCStr<'a> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		//CLONE CSTR DEBUG!
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
}

#[derive(Debug)]
pub struct DisplayCString {
	cstr: CString,
}

impl<'a> DisplayCString {
	#[inline]
	pub fn new(cstr: CString) -> Self {
		Self {
			cstr: cstr
		}
	}
}

impl Display for DisplayCString {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		//CLONE CSTR DEBUG!
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
}