
/*!
Safe display of CStr.
*/


use std::fmt::Debug;
use std::ffi::CString;
use std::fmt::{self, Display};
use std::ffi::CStr;
use std::fmt::Write;


pub trait DisplayCStr: Display + Debug {
	fn iter_mut_result<'a, F: FnMut(u8) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T>;
	fn iter_result<'a, F: Fn(u8) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T>;

	fn iter_char_mut_result<'a, F: FnMut(char) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T>;
	fn iter_char_result<'a, F: Fn(char) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T>;

	fn iter_mut<F: FnMut(u8)>(&self, function: F);
	fn iter<F: Fn(u8)>(&self, function: F);

	#[inline(always)]
	fn iter_char_mut<F: FnMut(char)>(&self, mut function: F) {
		self.iter_mut(|a| {
			function(a as char);
		});
	}
	#[inline(always)]
	fn iter_char<F: Fn(char)>(&self, function: F) {
		self.iter(|a| {
			function(a as char);
		});
	}
}


#[derive(Debug)]
pub struct DisplaySliceCStr<'a> {
	cstr: &'a CStr,
}

impl<'a> DisplaySliceCStr<'a> {
	#[inline]
	pub fn new(cstr: &'a CStr) -> Self {
		Self {
			cstr: cstr
		}
	}
	pub fn new_impl(cstr: &'a CStr) -> impl DisplayCStr + 'a {
		Self {
			cstr: cstr
		}
	}

	#[inline]
	pub fn into_result(self) -> &'a CStr {
		self.cstr
	}
}

impl<'l> DisplayCStr for DisplaySliceCStr<'l> {
	#[inline]
	fn iter_mut_result<'a, F: FnMut(u8) -> Result<(), T>, T: 'a>(&self, mut function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_result<'a, F: Fn(u8) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_char_result<'a, F: Fn(char) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a as char) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_char_mut_result<'a, F: FnMut(char) -> Result<(), T>, T: 'a>(&self, mut function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a as char) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_mut<F: FnMut(u8)>(&self, mut function: F) {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				function(a);
			}
		}
	}
	#[inline]
	fn iter<F: Fn(u8)>(&self, function: F) {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				function(a);
			}
		}
	}
}

impl<'a> Display for DisplaySliceCStr<'a> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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
	pub fn new_impl(cstr: CString) -> impl DisplayCStr + 'a {
		Self {
			cstr: cstr
		}
	}

	#[inline]
	pub fn into_result(self) -> CString {
		self.cstr
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

impl DisplayCStr for DisplayCString {
	#[inline]
	fn iter_mut_result<'a, F: FnMut(u8) -> Result<(), T>, T: 'a>(&self, mut function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_result<'a, F: Fn(u8) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_char_result<'a, F: Fn(char) -> Result<(), T>, T: 'a>(&self, function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a as char) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_char_mut_result<'a, F: FnMut(char) -> Result<(), T>, T: 'a>(&self, mut function: F) -> Result<(), T> {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				if let Err(e) = function(a as char) {
					return Err(e);
				}
			}
		}

		Ok( () )
	}

	#[inline]
	fn iter_mut<F: FnMut(u8)>(&self, mut function: F) {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				function(a);
			}
		}
	}
	#[inline]
	fn iter<F: Fn(u8)>(&self, function: F) {
		let array = self.cstr.to_bytes();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				function(a);
			}
		}
	}
}