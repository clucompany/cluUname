#![feature(const_fn)]

//Copyright 2019 #UlinProject Денис Котляров

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
use std::io::Error;

mod hash;
pub use self::hash::*;

mod display;
pub use self::display::*;

mod uts_struct;
pub use self::uts_struct::*;

mod element;
pub use self::element::*;

mod type_element;
pub use self::type_element::*;

///Getting information about the system.
pub mod build {
	use crate::uts_struct::UtsNameAlwaysType;
	use crate::element::UtsElement;
	use crate::uts_struct::UtsName;
	
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
	#[inline(always)]
	pub const fn custom<Q,W,E,R,T,Y>(a1: Q, a2: W, a3: E, a4: R, a5: T, a6: Y) -> UtsName<Q,W,E,R,T,Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
		UtsName::new(a1, a2, a3, a4, a5, a6)
	}
	
	///Create user information about the system
	///
	///sysname:	a1
	///nodename:	a2
	///release:	a3
	///version:	a4
	///machine:	a5
	///
	///#[cfg(feature = "enable_domainname")]
	///domainname:	a6
	///
	#[cfg(not(feature = "enable_domainname"))]
	#[inline(always)]
	pub const fn custom<Q,W,E,R,T>(a1: Q, a2: W, a3: E, a4: R, a5: T)-> UtsName<Q,W,E,R,T>  where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
		UtsName::new(a1, a2, a3, a4, a5)
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
	pub const fn linux_216_86() -> UtsNameAlwaysType<&'static str> {
		custom (
			"Linux",
			"cluComp",
			"2.16-localhost",
			"#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008",
			"x86",
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
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
	pub const fn linux_415_86_64() -> UtsNameAlwaysType<&'static str> {
		custom (
			"Linux",
			"cluComp",
			"4.15.15-1-zen",
			"#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018",
			"x86_64",
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
		)
	}
	
	pub const fn linux_420_86_64() -> UtsNameAlwaysType<&'static str> {
		custom (
			"Linux",
			"cluComp",
			"4.20.11-1-MANJARO",
			"#1 SMP PREEMPT Wed Feb 20 23:19:36 UTC 2019",
			"x86_64",
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
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
#[inline(always)]
pub fn uname() -> Result<UtsNameThisMachine, Error> {
	UtsNameThisMachine::this_machine()
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
#[inline(always)]
pub const fn custom_uname<Q, W, E, R, T, Y>(a1: Q, a2: W, a3: E, a4: R, a5: T, a6: Y) -> UtsName<Q, W, E, R, T, Y> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement, Y: UtsElement {
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
#[inline(always)]
pub const fn custom_uname<Q,W,E,R,T>(a1: Q, a2: W, a3: E, a4: R, a5: T) -> UtsName<Q,W,E,R,T>  where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	build::custom(a1, a2, a3, a4, a5)
}

/*
#[inline(always)]
pub fn uname_hash(uts: &UtsName) -> u64 {
	uts.uname_hash()
}

#[inline(always)]
pub fn version_hash(uts: &UtsName) -> u64 {
	uts.version_hash()
}
*/



#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	#[cfg(target_os = "linux")]
	fn linux() {
		if let Ok(uts) = uname() {
			assert_eq!(*uts.as_sysname(), b"Linux"[..]);
		}
	}
	
	#[test]
	fn custom() {
		let uts = custom_uname (
			"Linux",
			"cluComp",
			"4.15.15-1-zen",
			"#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018",
			"x86_64",
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
		);
		
		assert_eq!(uts.as_sysname(), "Linux");
		assert_eq!(uts.as_nodename(), "cluComp");
		assert_eq!(uts.as_release(), "4.15.15-1-zen");
		assert_eq!(uts.as_version(), "#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018");
		assert_eq!(uts.as_machine(), "x86_64");
		
		#[cfg(feature = "enable_domainname")]
		assert_eq!(uts.as_domainname(), "(none)");
	}
	
	#[test]
	fn hash() {
		let uts_str = custom_uname (
			"Linux",
			"",
			"4.15.15-1-zen",
			"#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018",
			"",
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
		);
		
		
		let uts_vec_byte = custom_uname (
			"Linux".as_bytes().to_vec(),
			&None::<&str>,
			&b"4.15.15-1-zen"[..],
			"#1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018",
			(),
			
			#[cfg(feature = "enable_domainname")]
			"(none)",
		);
		
		assert_eq!(uts_str.uname_hash(), uts_vec_byte.uname_hash());
		assert_eq!(uts_str.version_hash(), uts_vec_byte.version_hash());
	}
}
