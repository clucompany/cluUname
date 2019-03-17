
use std::ffi::CString;
use std::ffi::CStr;


#[derive(Debug, Clone)]
pub enum UtsType<'a> {
	U8(&'a [u8]),
	Cstr(&'a CStr),
	Str(&'a str),
	
	Option(UtsOptionType<'a>)
}

impl<'a> UtsType<'a> {
	pub fn is_utf8(&self) -> bool {
		match self {
			UtsType::Str(_a) => true,
			_ => false,
		}
	}
	#[inline(always)]
	pub fn is_raw(&self) -> bool {
		!self.is_utf8()
	}
	
	pub fn is_option(&self) -> bool {
		match self {
			UtsType::Option(_a) => true,
			_ => false,
		}	
	}
}

impl<'a, 'l, T: Into<UtsType<'a>>> From<&'l T> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'l T) -> Self {
		a.into()
	}
}

impl<'a> From<()> for UtsType<'a> {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		UtsType::U8(b"")
	}
}
impl<'a> From<&'a Vec<u8>> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a Vec<u8>) -> Self {
		UtsType::U8(a)
	}
}

impl<'a> From<&'a [u8]> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a [u8]) -> Self {
		UtsType::U8(a)
	}
}

impl<'a> From<&'a CStr> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a CStr) -> Self {
		UtsType::Cstr(a)
	}
}

impl<'a> From<&'a CString> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a CString) -> Self {
		UtsType::Cstr(a)
	}
}

impl<'a> From<&'a str> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a str) -> Self {
		UtsType::Str(a)
	}
}

impl<'a> From<&'a String> for UtsType<'a> {
	#[inline(always)]
	fn from(a: &'a String) -> Self {
		UtsType::Str(a)
	}
}

impl<'a> From<UtsOptionType<'a>> for UtsType<'a> {
	#[inline(always)]
	fn from(a: UtsOptionType<'a>) -> Self {
		UtsType::Option(a)
	}	
}

impl<'a, T: Into<UtsOptionType<'a>>> From<Option<T>> for UtsType<'a> {
	fn from(a: Option<T>) -> Self {
		UtsType::Option(a.into())
	}
}


#[derive(Debug, Clone)]
pub enum UtsOptionType<'a> {
	U8(&'a [u8]),
	Cstr(&'a CStr),
	Str(&'a str),
	
	None,
}

impl<'a, 'l, T: Into<UtsOptionType<'a>>> From<&'l T> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: &'l T) -> Self {
		a.into()
	}
}

impl<'a, T: Into<UtsOptionType<'a>>> From<Option<T>> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: Option<T>) -> Self {
		match a {
			Some(a) => a.into(),
			_ => ().into(),
		}
	}
}

impl<'a> From<&'a [u8]> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: &'a [u8]) -> Self {
		UtsOptionType::U8(a)
	}
}

impl<'a> From<&'a CStr> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: &'a CStr) -> Self {
		UtsOptionType::Cstr(a)
	}
}

impl<'a> From<&'a str> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: &'a str) -> Self {
		UtsOptionType::Str(a)
	}
}

impl<'a> From<&'a String> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(a: &'a String) -> Self {
		UtsOptionType::Str(a)
	}
}


impl<'a> From<()> for UtsOptionType<'a> {
	#[inline(always)]
	fn from(_a: ()) -> Self {
		UtsOptionType::None
	}
}

