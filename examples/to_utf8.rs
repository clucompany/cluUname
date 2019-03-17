

use std::ffi::CStr;

extern crate cluuname;
use cluuname::build::custom;
use std::ffi::CString;

pub fn main() {
	{
		let raw_cstr = custom (
			CStr::from_bytes_with_nul(b"Linux\0").unwrap(),
			CStr::from_bytes_with_nul(b"\0").unwrap(),
			CStr::from_bytes_with_nul(b"2.16-localhost\0").unwrap(),	// <<<
			CStr::from_bytes_with_nul(b"#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008\0").unwrap(), // <<<
			CString::new("x86").unwrap(),
			
			#[cfg(feature = "enable_domainname")]
			CStr::from_bytes_with_nul(b"(none)\0").unwrap(),
		);
		//CSTR
		
		println!("{}", raw_cstr);
		let raw_hash = raw_cstr.uname_hash();
		
		let str = raw_cstr.to_utf8().unwrap();
		//String
		
		println!("{}", str);
		
		assert_eq!(raw_hash, str.uname_hash());
	}
}
