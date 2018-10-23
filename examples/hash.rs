

#![feature(plugin)]
#![plugin(clucstr)]
#[allow(plugin_as_library)]
extern crate clucstr;
use std::ffi::CStr;

extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;
use cluuname::build::custom;

pub fn main() {
	{//1
		let uname = uname().unwrap();
		
		println!("HASH {}", uname.uname_hash());
		println!("HASH_VERSION {}", uname.version_hash());
		
	}
	
	
	{//2
		let hash_version_test = custom (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("2.16-localhost"),	// <<<
			cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"), // <<<
			cstr!("x86"),
			
			#[cfg(feature = "enable_domainname")]
			cstr!("(none)"),
		).version_hash();
		
		
		let hash_version_test_1 = custom (
			cstr!("Linux"),
			cstr!(""),
			cstr!("2.16-localhost"),
			cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
			cstr!(""),
			
			#[cfg(feature = "enable_domainname")]
			cstr!(""),
		).version_hash();
		
		assert_eq!(hash_version_test, hash_version_test_1);
	}
}
