
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use cluuname::core::UnameErr;
use cluuname::custom_rs_uname;

fn main() -> Result<(), UnameErr> {
	let cstr_array = &b"34Test\0"[..];
	
	let uname = custom_rs_uname(
		"1Test",
		&b"2Test"[..],
		unsafe { CStr::from_ptr(cstr_array.as_ptr() as _) },
		unsafe { CStr::from_ptr(cstr_array.as_ptr() as _).to_owned() as CString },
		String::from("5Test"),
		Cow::Borrowed("6Test"),
	);
	
	println!("{:?}", uname);
	println!("{}", uname);
	drop(cstr_array);
	
	Ok(())
}
