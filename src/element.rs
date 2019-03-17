
use crate::type_element::UtsOptionType;
use crate::type_element::UtsType;
use std::string::FromUtf8Error;
use std::str::Utf8Error;
use std::ffi::IntoStringError;
use std::fmt::Display;
use std::ffi::CString;
use std::fmt::Debug;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Write;

#[derive(Debug)]
pub enum UTSUTF8Err {
	FFI(IntoStringError),
	UTF8(Utf8Error),
	FromUtf8(FromUtf8Error),
}

impl From<IntoStringError> for UTSUTF8Err {
	#[inline(always)]
	fn from(a: IntoStringError) -> Self {
		UTSUTF8Err::FFI(a)	
	}
}
impl From<Utf8Error> for UTSUTF8Err {
	#[inline(always)]
	fn from(a: Utf8Error) -> Self {
		UTSUTF8Err::UTF8(a)	
	}
}
impl From<FromUtf8Error> for UTSUTF8Err {
	#[inline(always)]
	fn from(a: FromUtf8Error) -> Self {
		UTSUTF8Err::FromUtf8(a)	
	}
}



#[inline(always)]
fn result_into_uerr<T: Into<R>, E: Into<UTSUTF8Err>, R>(a: Result<T, E>) -> Result<R, UTSUTF8Err> {
	match a {
		Ok(a)	=> Ok(a.into()),
		Err(e) => Err(e.into())
	}
}



pub trait UtsElement: Debug {
	fn as_array(&self) -> &[u8];

	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result;
	
	/*fn display(&self) -> DisplayUts<&Self> where Self: Sized {
		DisplayUts::from(self)
	}*/
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t>;
}


pub trait UtsElementIntoUTF {
	fn into_utf8(self) -> Result<String, UTSUTF8Err>;
}


impl<'a> Display for dyn UtsElement + 'a {
	#[inline(always)]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.display_fmt(fmt)
	}
}

impl<'a, A: UtsElement> UtsElement for &'a A {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		A::as_array(self)
	}
	
	#[inline(always)]
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		A::display_fmt(self, fmt)
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		A::type_as(self)
	}
}

impl<'a, 'l> UtsElement for &'a (dyn UtsElement + 'l) {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		(**self).as_array()
	}
	
	#[inline(always)]
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		(**self).display_fmt(fmt)
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		(**self).type_as()
	}
}



impl<'a> UtsElement for &'a CStr {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self.to_bytes()
	}
	
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let array = self.as_array();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )	
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}
impl<'a> UtsElementIntoUTF for &'a CStr {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		result_into_uerr(self.to_str())	
	}
}


impl<'a> UtsElement for &'a [u8] {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self
	}
	
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let array = self.as_array();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )	
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl<'a> UtsElementIntoUTF for &'a [u8] {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		result_into_uerr(std::str::from_utf8(self))
	}
}



impl UtsElement for Vec<u8> {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self
	}
	
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let array = self.as_array();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl UtsElementIntoUTF for Vec<u8> {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		result_into_uerr(String::from_utf8(self))
	}
}



impl UtsElement for CString {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self.to_bytes()
	}
	
	#[inline(always)]
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let array = self.as_array();

		for a in array.into_iter() {
			for a in ::std::ascii::escape_default(*a) {
				fmt.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl UtsElementIntoUTF for CString {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		result_into_uerr(String::from_utf8(self.into_bytes()))
	}
}


impl<'a> UtsElement for &'a str {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self.as_bytes()
	}
	
	#[inline(always)]
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str(self)
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl<'a> UtsElementIntoUTF for &'a str {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		Ok(self.to_string())
	}
}

impl UtsElement for String {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		self.as_bytes()
	}
	
	#[inline(always)]
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str(self)
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl UtsElementIntoUTF for String {
	#[inline(always)]
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		Ok(self)
	}
}


impl<'l, T: UtsElement + Into<UtsOptionType<'l>>> UtsElement for &'l Option<T> {
	fn as_array(&self) -> &[u8] {
		match self {
			Some(a) => T::as_array(a),
			_ => b"",
		}
	}
	
	fn display_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Some(a) => T::display_fmt(a, fmt),
			_ => Ok(()),
		}
	}
	
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		match self {
			Some(a) => UtsType::Option(UtsOptionType::from(a)),
			_ => UtsType::Option(().into())
		}
	}
}



impl<T: UtsElementIntoUTF> UtsElementIntoUTF for Option<T> {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		match self {
			Some(a) => (a).into_utf8(),
			_ => Ok("".to_string())
		}
	}
}





impl UtsElement for () {
	#[inline(always)]
	fn as_array(&self) -> &[u8] {
		b""
	}
	
	#[inline(always)]
	fn display_fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
		Ok( () )
	}
	
	#[inline(always)]
	fn type_as<'t>(&'t self) -> UtsType<'t> {
		self.into()
	}
}

impl UtsElementIntoUTF for () {
	fn into_utf8(self) -> Result<String, UTSUTF8Err> {
		Ok("".to_string())
	}
}


impl<'a> PartialEq<dyn UtsElement> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &dyn UtsElement) -> bool {
		PartialEq::eq(self.as_array(), a.as_array())
	}
}



impl<'a> PartialEq<()> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, _a: &()) -> bool {
		self.as_array().len() == 0
	}
}

impl<'a, 'l> PartialEq<Option<&'a CStr>> for dyn UtsElement + 'l {
	#[inline(always)]
	fn eq(&self, a: &Option<&'a CStr>) -> bool {
		match a {
			Some(a) => PartialEq::eq(self.as_array(), a.as_array()),
			_ => {
				match self.as_array().len() {
					0 => return true,
					_ => return false,
				}	
			},
		}
	}
}

impl<'a> PartialEq<Option<CString>> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &Option<CString>) -> bool {
		match a {
			Some(a) => PartialEq::eq(self.as_array(), a.as_array()),
			_ => {
				match self.as_array().len() {
					0 => return true,
					_ => return false,
				}	
			},
		}
	}
}

impl<'a> PartialEq<CString> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &CString) -> bool {
		PartialEq::eq(self.as_array(), a.as_array())
	}
}

impl<'a> PartialEq<CStr> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &CStr) -> bool {
		PartialEq::eq(self.as_array(), a.as_array())
	}
}

impl<'a> PartialEq<str> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &str) -> bool {
		PartialEq::eq(self.as_array(), a.as_array())
	}
}

impl<'a> PartialEq<String> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &String) -> bool {
		PartialEq::eq(self.as_array(), a.as_array())
	}
}

impl<'a> PartialEq<[u8]> for dyn UtsElement + 'a {
	#[inline(always)]
	fn eq(&self, a: &[u8]) -> bool {
		PartialEq::eq(self.as_array(), a)
	}
}