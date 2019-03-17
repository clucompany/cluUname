

use std::ffi::CStr;

extern crate cluuname;
use cluuname::uname;
use cluuname::build::custom;

pub fn main() {
	{//1
		let uname = uname().unwrap();
		
		println!("HASH {}", uname.uname_hash());
		println!("HASH_VERSION {}", uname.version_hash());
		
	}
	
	
	{//2
		let hash_version_test = custom (
			CStr::from_bytes_with_nul(b"Linux\0").unwrap(),
			CStr::from_bytes_with_nul(b"\0").unwrap(),
			CStr::from_bytes_with_nul(b"2.16-localhost\0").unwrap(),	// <<<
			CStr::from_bytes_with_nul(b"#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008\0").unwrap(), // <<<
			CStr::from_bytes_with_nul(b"x86\0").unwrap(),
			
			#[cfg(feature = "enable_domainname")]
			CStr::from_bytes_with_nul(b"(none)\0").unwrap(),
		);
		
		
		let hash_version_test_1 = custom (
			"Linux",
			&None::<&str>,
			"2.16-localhost",	// <<<
			"#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008", // <<<
			&b"x86"[..],
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
		);
		
		println!("{} == {}", hash_version_test, hash_version_test_1);
		assert_eq!(hash_version_test.uname_hash(), hash_version_test_1.uname_hash());
	}
}
