//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//


/*!
Name and information about the current kernel.

# Print
```
use cluUname::uname;

let uname = uname().unwrap();
println!("{}", uname);
//Linux cluComp 4.15.15-1-zen #1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018 x86_64 (none)
```

# CustomPrint
```
use cluUname::uname;
use cluUname::UtsName;

let uname = uname().unwrap();

let sysname = uname.as_sysname();
let nodename = uname.as_nodename();
let release = uname.as_release();
let version = uname.as_version();
let machine = uname.as_machine();
let domainname = uname.as_domainname();

println!("{:?} {:?} {:?} {:?} {:?} {:?}", sysname, nodename, release, version, machine, domainname);
//"Linux "cluComp" "4.15.15-1-zen" "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018" "x86_64" "(none)"
```

# Hash + Hash Version
```
use cluUname::uname;
use cluUname::UtsName;

let uname = uname().unwrap();

let machine_all_hash = uname.uname_hash();
let machive_version_hash = uname.version_hash();

println!("UNAME_HASH {}", machine_all_hash);
println!("UNAME_V_HASH {}", machive_version_hash);


```

# Custom

```
use cluUname::builder;
macro_rules! cstr {
	($s:expr) => {
		unsafe {
       		::std::ffi::CStr::from_ptr(
				concat!($s, "\0").as_ptr() as *const ::std::os::raw::c_char
			)
		}
	};
}

let uname = builder::custom(
	cstr!("Linux"),
	cstr!("cluComp"),
	cstr!("2.16-localhost"),
	cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
	cstr!("x86"),
	cstr!("(none)"),
);
println!("{}", uname);
//Linux cluComp 2.16-localhost #1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008 x86 (none)
```

*/



pub mod hash_version;
pub mod uts_struct;

use hash_version::HashVersion;
use uts_struct::buf::UtsNameBuf;
use uts_struct::slice::UtsNameSlice;

use std::fmt::Debug;
use std::ffi::CStr;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


macro_rules! cstr {
	($s:expr) => {
		unsafe {
       		::std::ffi::CStr::from_ptr(
				concat!($s, "\0").as_ptr() as *const ::std::os::raw::c_char
			)
		}
	};
}

///Basic uname trait
pub trait UtsName: Hash + HashVersion + Debug + PartialEq + Eq + PartialOrd + Clone {
	///Get sysname for this structure.
	fn as_sysname(&self) -> &CStr;
	///Get nodename for this structure.
	fn as_nodename(&self) -> &CStr;
	///Get release for this structure.
	fn as_release(&self) -> &CStr;
	///Get version for this structure.
	fn as_version(&self) -> &CStr;
	///Get machine for this structure.
	fn as_machine(&self) -> &CStr;
	///Get domainname for this structure.
	fn as_domainname(&self) -> &CStr;
	
	
	fn uname_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish()  
	}
	fn version_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.hash_version(&mut hasher);
		hasher.finish()  
	}
}







///Getting and creating a custom uname
pub mod builder {
	use uts_struct::slice::UtsNameSlice;
	use uts_struct::buf::UtsNameBuf;
	use std::ffi::CStr;
	
	///Create custom uname
	#[inline]
	pub fn custom<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> UtsNameSlice<'a> {
		UtsNameSlice::new(a1, a2, a3, a4, a5, a6)
	}
	
	
	///Getting the current uname
	#[inline]
	pub fn this_machine() -> Result<UtsNameBuf, i32> {
		UtsNameBuf::this_machine()
	}
	
	///"Linux" "cluComp" "2.16-localhost" "#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008" "x86" "(none)"
	pub fn linux_216_86<'a>() -> UtsNameSlice<'a> {
		custom (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("2.16-localhost"),
			cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
			cstr!("x86"),
			cstr!("(none)"),
		)
	}
	
	///"Linux" "cluComp" "4.15.15-1-zen" "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018" "x86_64" "(none)"
	pub fn linux_415_86_64<'a>() -> UtsNameSlice<'a> {
		custom (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("4.15.15-1-zen"),
			cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"),
			cstr!("x86_64"),
			cstr!("(none)"),
		)
	}
	
	
}


///Getting the current uname
#[inline]
pub fn uname() -> Result<UtsNameBuf, i32> {
	builder::this_machine()
}

///Create custom uname
#[inline]
pub fn custom_uname<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> UtsNameSlice<'a> {
	builder::custom(a1, a2, a3, a4, a5, a6)
}


#[inline]
pub fn uname_hash<I: UtsName>(uts: &I) -> u64 {
	uts.uname_hash()
}

#[inline]
pub fn version_hash<I: UtsName>(uts: &I) -> u64 {
	uts.version_hash()
}




#[cfg(test)]
mod tests {
	use super::*;
	
	
	#[test]
	#[cfg(target_os = "linux")]
	fn linux() {
		let uts = uname().unwrap();
		
		assert_eq!(uts.as_sysname(), cstr!("Linux"));
	}
	
	#[test]
	fn custom() {
		let uts = custom_uname (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("4.15.15-1-zen"),
			cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"),
			cstr!("x86_64"),
			cstr!("(none)"),
		);
		
		assert_eq!(uts.as_sysname(), cstr!("Linux"));
		assert_eq!(uts.as_nodename(), cstr!("cluComp"));
		assert_eq!(uts.as_release(), cstr!("4.15.15-1-zen"));
		assert_eq!(uts.as_version(), cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"));
		assert_eq!(uts.as_machine(), cstr!("x86_64"));
		assert_eq!(uts.as_domainname(), cstr!("(none)"));
	}
}


