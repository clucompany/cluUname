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
Library for displaying information about the system. Implemented only for Linux.

# Print
```
extern crate cluuname;
use cluuname::uname;

fn main() {
	let uname = uname().unwrap();
	println!("{}", uname);
	
	//Linux cluComp 4.15.15-1-zen #1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018 x86_64
}
```
# 2Print

```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;
use cluuname::build;

fn main() {
	let uname = uname().unwrap();
	nodename(uname);
	//NODENAME "R510"
	
	let custom_uname = build::linux_216_86();
	nodename(custom_uname);
	//NODENAME "cluComp"
}

fn nodename<T: UtsName>(uname: T) {
	println!("NODENAME {}", uname.display_nodename());
}
```


# CustomPrint
```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;

fn main() {
	let uname = uname().unwrap();

	let sysname = uname.display_sysname();
	let nodename = uname.display_nodename();
	let release = uname.display_release();
	let version = uname.display_version();
	let machine = uname.display_machine();

	println!("{} {} {} {} {}", sysname, nodename, release, version, machine);
	//"Linux "cluComp" "4.15.15-1-zen" "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018" "x86_64"
}
```

# Hash + Hash Version
```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;

fn main() {
    let uname = uname().unwrap();

    let machine_all_hash = uname.uname_hash();
    let machive_version_hash = uname.version_hash();

    println!("UNAME_HASH {}", machine_all_hash);
    //12821596144084292007
    println!("UNAME_V_HASH {}", machive_version_hash);
    //2978006705337010168
}
```

# CustomUname

```
#![feature(plugin)]
#![plugin(clucstr)]
extern crate cluuname;
use cluuname::build;

use std::ffi::CStr;

fn main() {
	let uname = build::custom(
		cstr!("Linux"),
		cstr!("cluComp"),
		cstr!("2.16-localhost"),
		cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
		cstr!("x86"),
	);
	println!("{}", uname);
	//Linux cluComp 2.16-localhost #1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008 x86
}
```

# Flags
enable_domainname - Additional item `domainname`

```
[dependencies]
cluuname = { version = "*", features = ["enable_domainname"] }
```

*/
#![feature(plugin)]
#![plugin(clucstr)]

use std::ffi::CStr;


pub mod hash;
pub mod uts_struct;
pub mod display_cstr;

use hash::HashVersion;

use std::fmt::Debug;
use std::fmt::Display;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use display_cstr::DisplayCStr;

///Basic uname trait
pub trait UtsName: Hash + HashVersion + Display + Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Clone {
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
	#[cfg(feature = "enable_domainname")]
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
	
	#[inline]
	///Display trait for sysname.
	fn display_sysname<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_sysname())
	}
	
	///Display trait for nodename.
	#[inline]
	fn display_nodename<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_nodename())
	}
	
	///Display trait for release.
	#[inline]
	fn display_release<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_release())
	}
	
	///Display trait for version.
	#[inline]
	fn display_version<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_version())
	}
	
	///Display trait for machine.
	#[inline]
	fn display_machine<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_machine())
	}

	///Display trait for domainname.
	#[cfg(feature = "enable_domainname")]
	#[inline]
	fn display_domainname<'r>(&'r self) -> DisplayCStr<'r> {
		DisplayCStr::new(self.as_domainname())
	}
	
}







///Getting information about the system.
pub mod build {
	use uts_struct::slice::UtsNameSlice;
	use uts_struct::buf::UtsNameBuf;
	use std::ffi::CStr;
	use UtsName;
	
	
	///Create user information about the system
	///```
	///sysname:	a1
	///nodename:	a2
	///release:	a3
	///version:	a4
	///machine:	a5
	///
	///#[cfg(feature = "enable_domainname")]
	///domainname:	a6
	///```
	#[cfg(feature = "enable_domainname")]
	#[inline]
	pub fn custom<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> impl UtsName + 'a {
		UtsNameSlice::new(a1, a2, a3, a4, a5, a6)
	}
	
	///Create user information about the system
	///```
	///sysname:	a1
	///nodename:	a2
	///release:	a3
	///version:	a4
	///machine:	a5
	///
	///#[cfg(feature = "enable_domainname")]
	///domainname:	a6
	///```
	#[cfg(not(feature = "enable_domainname"))]
	#[inline]
	pub fn custom<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr) -> impl UtsName + 'a {
		UtsNameSlice::new(a1, a2, a3, a4, a5)
	}
		
	///Getting system information about the current machine
	#[inline]
	pub fn this_machine() -> Result<impl UtsName, i32> {
		UtsNameBuf::this_machine()
	}
	
	///"Linux" "cluComp" "2.16-localhost" "#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008" "x86" "(none)"
	///```
	///sysname:	cstr!("Linux")
	///nodename:	cstr!("cluComp")
	///release:	cstr!("2.16-localhost")
	///version:	cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008")
	///machine:	cstr!("x86")
	///
	///#[cfg(feature = "enable_domainname")]
	///domainname:	cstr!("(none)")
	///```
	pub fn linux_216_86() -> impl UtsName + 'static {
		custom (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("2.16-localhost"),
			cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
			cstr!("x86"),
			
			#[cfg(feature = "enable_domainname")]
			cstr!("(none)"),
		)
	}
	
	///"Linux" "cluComp" "4.15.15-1-zen" "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018" "x86_64" "(none)"
	///```
	///sysname:	cstr!("Linux")
	///nodename:	cstr!("cluComp")
	///release:	cstr!("4.15.15-1-zen")
	///version:	cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018")
	///machine:	cstr!("x86_64")
	///
	///#[cfg(feature = "enable_domainname")]
	///domainname:	cstr!("(none)")
	///```
	///
	pub fn linux_415_86_64() -> impl UtsName + 'static {
		custom (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("4.15.15-1-zen"),
			cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"),
			cstr!("x86_64"),
			
			#[cfg(feature = "enable_domainname")]
			cstr!("(none)"),
		)
	}
	
	
}


///Getting system information about the current machine
///```
///extern crate cluuname;
///use cluuname::uname;
///
///fn main() {
///	let uname = uname().unwrap();
///	println!("{}", uname);
///	//"Linux" "cluComp" "4.15.15-1-zen" "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018" "x86_64"
///}
#[inline]
pub fn uname() -> Result<impl UtsName, i32> {
	build::this_machine()
}

///Create user information about the system
///```
///sysname:	a1
///nodename:	a2
///release:	a3
///version:	a4
///machine:	a5
///
///#[cfg(feature = "enable_domainname")]
///domainname:	a6
///```
#[cfg(feature = "enable_domainname")]
#[inline]
pub fn custom_uname<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> impl UtsName + 'a {
	build::custom(a1, a2, a3, a4, a5, a6)
}

///Create user information about the system
///```
///sysname:	a1
///nodename:	a2
///release:	a3
///version:	a4
///machine:	a5
///
///#[cfg(feature = "enable_domainname")]
///domainname:	a6
///```
#[cfg(not(feature = "enable_domainname"))]
#[inline]
pub fn custom_uname<'a>(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr) -> impl UtsName + 'a {
	build::custom(a1, a2, a3, a4, a5)
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
		if let Ok(uts) = uname() {
			assert_eq!(uts.as_sysname(), cstr!("Linux"));
		}
	}
	
	#[test]
	fn custom() {
		let uts = custom_uname (
			cstr!("Linux"),
			cstr!("cluComp"),
			cstr!("4.15.15-1-zen"),
			cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"),
			cstr!("x86_64"),
			
			#[cfg(feature = "enable_domainname")]
			cstr!("(none)"),
		);
		
		assert_eq!(uts.as_sysname(), cstr!("Linux"));
		assert_eq!(uts.as_nodename(), cstr!("cluComp"));
		assert_eq!(uts.as_release(), cstr!("4.15.15-1-zen"));
		assert_eq!(uts.as_version(), cstr!("#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018"));
		assert_eq!(uts.as_machine(), cstr!("x86_64"));
		
		#[cfg(feature = "enable_domainname")]
		assert_eq!(uts.as_domainname(), cstr!("(none)"));
	}
}


