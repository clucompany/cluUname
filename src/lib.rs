//Copyright 2019-2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022
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
#![no_std]

#![allow(non_snake_case)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use ::core::ffi::CStr;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::Write;
#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
#[cfg( any(test, feature = "box") )]
use crate::beh::def_box_linux::BoxArrayLinuxUTSName;
#[cfg_attr(docsrs, doc(cfg(feature = "cstring")))]
#[cfg( any(test, feature = "cstring") )]
use crate::beh::def_cstring_linux::CStringLinuxUTSName;
#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
use crate::beh::def_rs_linux::StringLinuxUTSName;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::beh::def_linux::LinuxUTSNameType;
use crate::beh::def_linux::RawLinuxUTSNameType;
use crate::core::AsUname;
use crate::core::AsPtrUname;
use crate::core::UnameData;
use crate::core::UnameErr;
use ::core::hash::Hash;
//use ::core::fmt::Debug;

pub mod beh {
	pub mod def_linux;
	#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
	#[cfg( any(test, feature = "box") )]
	pub mod def_box_linux;
	#[cfg_attr(docsrs, doc(cfg(feature = "cstring")))]
	#[cfg( any(test, feature = "cstring") )]
	pub mod def_cstring_linux;
	#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
	#[cfg( any(test, feature = "rs") )]
	pub mod def_rs_linux;
}

mod macro_make;
pub mod core;

#[repr(transparent)]
pub struct Uname<D> where D: UnameData {
	data: D::Data,
}

/*impl<D> Debug for Uname<D> where D: UnameData {
	fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
		Debug::fmt(&self.data, f)
	}
}*/

impl<D> Hash for Uname<D> where D: UnameData {
	#[inline(always)]
	fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
		D::hash_data(self.as_data(), state)
	}
}

impl<D> Uname<D> where D: UnameData {
	#[inline(always)]
	pub fn get_current() -> Result<Uname<D>, UnameErr> {
		D::get_current()
	}
	
	#[inline(always)]
	pub fn get_current_fn<R>(ok: impl FnOnce(Uname<D>) -> R, e: impl FnOnce(UnameErr) -> R) -> R {
		D::get_current_fn(ok, e)
	}
	
	#[inline(always)]
	pub fn get_current_or_empty() -> Uname<D> {
		D::get_current_or_empty()
	}
	
	#[inline]
	pub const fn from(data: D::Data) -> Self {
		Self {
			data,
		}
	}
	
	#[inline(always)]
	pub const fn as_data(&self) -> &D::Data {
		&self.data
	}
	
	#[inline(always)]
	pub /*const*/ fn as_mut_data(&mut self) -> &mut D::Data {
		&mut self.data
	}
}

#[repr(transparent)]
struct _SlowSafeDisplay<'a>(&'a [u8]);

impl<'a> Display for _SlowSafeDisplay<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
		for a in self.0.into_iter() {
			for a in ::core::ascii::escape_default(*a) {
				f.write_char(a as char)?;
			}
		}
		
		Ok( () )
	}
}

impl<D> Uname<D> where D: UnameData + AsUname<[u8], Data = <D as UnameData>::Data> {
	pub fn slow_write_to<W: Write>(&self, w: &mut W) -> ::core::fmt::Result {
		write!(
			w, "{} {} {} {} {} {}", 
			_SlowSafeDisplay(self.as_sysname()), 
			_SlowSafeDisplay(self.as_nodename()),
			_SlowSafeDisplay(self.as_release()),
			_SlowSafeDisplay(self.as_version()),
			_SlowSafeDisplay(self.as_machine()),
			_SlowSafeDisplay(self.as_domainname()),
		)
	}
	
	pub fn get_slow_display<'a>(&'a self) -> impl Display + 'a {
		struct __FullDisplay<'a, D>(&'a Uname<D>) where D: UnameData + AsUname<[u8], Data = <D as UnameData>::Data>;
		impl<'a, D> Display for __FullDisplay<'a, D> where D: UnameData + AsUname<[u8], Data = <D as UnameData>::Data> {
			fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
				write!(
					f, "{} {} {} {} {} {}", 
					_SlowSafeDisplay(self.0.as_sysname()), 
					_SlowSafeDisplay(self.0.as_nodename()),
					_SlowSafeDisplay(self.0.as_release()),
					_SlowSafeDisplay(self.0.as_version()),
					_SlowSafeDisplay(self.0.as_machine()),
					_SlowSafeDisplay(self.0.as_domainname()),
				)
			}
		}
		__FullDisplay(self)
	}
	
	#[inline(always)]
	pub fn as_sysname<'a>(&'a self) -> &'a [u8] {
		D::as_sysname(&self.data)
	}
	
	#[inline(always)]
	pub fn as_nodename<'a>(&'a self) -> &'a [u8] {
		D::as_nodename(&self.data)
	}
	
	#[inline(always)]
	pub fn as_release<'a>(&'a self) -> &'a [u8] {
		D::as_release(&self.data)
	}
	
	#[inline(always)]
	pub fn as_version<'a>(&'a self) -> &'a [u8] {
		D::as_version(&self.data)
	}
	
	#[inline(always)]
	pub fn as_machine<'a>(&'a self) -> &'a [u8] {
		D::as_machine(&self.data)
	}
	
	#[inline(always)]
	pub fn as_domainname<'a>(&'a self) -> &'a [u8] {
		D::as_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameData + AsUname<CStr, Data = <D as UnameData>::Data> {
	#[inline(always)]
	pub fn as_cstr_sysname<'a>(&'a self) -> &'a CStr {
		D::as_sysname(&self.data)
	}
	
	#[inline(always)]
	pub fn as_cstr_nodename<'a>(&'a self) -> &'a CStr {
		D::as_nodename(&self.data)
	}
	
	#[inline(always)]
	pub fn as_cstr_release<'a>(&'a self) -> &'a CStr {
		D::as_release(&self.data)
	}
	
	#[inline(always)]
	pub fn as_cstr_version<'a>(&'a self) -> &'a CStr {
		D::as_version(&self.data)
	}
	
	#[inline(always)]
	pub fn as_cstr_machine<'a>(&'a self) -> &'a CStr {
		D::as_machine(&self.data)
	}
	
	#[inline(always)]
	pub fn as_cstr_domainname<'a>(&'a self) -> &'a CStr {
		D::as_domainname(&self.data)
	}
}

impl<D> Display for Uname<D> where D: UnameData + AsUname<str, Data = <D as UnameData>::Data> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> ::core::fmt::Result {
		Display::fmt(&self.get_display(), f)
	}
}

impl<D> Uname<D> where D: UnameData + AsUname<str, Data = <D as UnameData>::Data> {
	#[inline]
	pub fn write_to<W: Write>(&self, w: &mut W) -> ::core::fmt::Result {
		write!(
			w, "{} {} {} {} {} {}", 
			self.as_str_sysname(), 
			self.as_str_nodename(),
			self.as_str_release(),
			self.as_str_version(),
			self.as_str_machine(),
			self.as_str_domainname(),
		)
	}
	
	pub fn get_display<'a>(&'a self) -> impl Display + 'a {
		struct __FullDisplay<'a, D>(&'a Uname<D>) where D: UnameData + AsUname<str, Data = <D as UnameData>::Data>;
		impl<'a, D> Display for __FullDisplay<'a, D> where D: UnameData + AsUname<str, Data = <D as UnameData>::Data> {
			fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
				write!(
					f, "{} {} {} {} {} {}", 
					self.0.as_str_sysname(), 
					self.0.as_str_nodename(),
					self.0.as_str_release(),
					self.0.as_str_version(),
					self.0.as_str_machine(),
					self.0.as_str_domainname(),
				)
			}
		}
		__FullDisplay(self)
	}
	
	#[inline(always)]
	pub fn as_str_sysname<'a>(&'a self) -> &'a str {
		D::as_sysname(&self.data)
	}
	
	#[inline(always)]
	pub fn as_str_nodename<'a>(&'a self) -> &'a str {
		D::as_nodename(&self.data)
	}
	
	#[inline(always)]
	pub fn as_str_release<'a>(&'a self) -> &'a str {
		D::as_release(&self.data)
	}
	
	#[inline(always)]
	pub fn as_str_version<'a>(&'a self) -> &'a str {
		D::as_version(&self.data)
	}
	
	#[inline(always)]
	pub fn as_str_machine<'a>(&'a self) -> &'a str {
		D::as_machine(&self.data)
	}
	
	#[inline(always)]
	pub fn as_str_domainname<'a>(&'a self) -> &'a str {
		D::as_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameData + AsPtrUname<LinuxUTSNameType, Data = <D as UnameData>::Data> {
	#[inline(always)]
	pub fn as_ptr_sysname(&self) -> *const LinuxUTSNameType {
		D::as_ptr_sysname(&self.data)
	}
	
	#[inline(always)]
	pub fn as_ptr_nodename(&self) -> *const LinuxUTSNameType {
		D::as_ptr_nodename(&self.data)
	}
	
	#[inline(always)]
	pub fn as_ptr_release(&self) -> *const LinuxUTSNameType {
		D::as_ptr_release(&self.data)
	}
	
	#[inline(always)]
	pub fn as_ptr_version(&self) -> *const LinuxUTSNameType {
		D::as_ptr_version(&self.data)
	}
	
	#[inline(always)]
	pub fn as_ptr_machine(&self) -> *const LinuxUTSNameType {
		D::as_ptr_machine(&self.data)
	}
	
	#[inline(always)]
	pub fn as_ptr_domainname(&self) -> *const LinuxUTSNameType {
		D::as_ptr_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameData + AsPtrUname<RawLinuxUTSNameType, Data = <D as UnameData>::Data> {
	#[inline(always)]
	pub fn as_rawptr_sysname(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_sysname(&self.data)
	}
	
	#[inline(always)]
	pub fn as_rawptr_nodename(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_nodename(&self.data)
	}
	
	#[inline(always)]
	pub fn as_rawptr_release(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_release(&self.data)
	}
	
	#[inline(always)]
	pub fn as_rawptr_version(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_version(&self.data)
	}
	
	#[inline(always)]
	pub fn as_rawptr_machine(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_machine(&self.data)
	}
	
	#[inline(always)]
	pub fn as_rawptr_domainname(&self) -> *const RawLinuxUTSNameType {
		D::as_ptr_domainname(&self.data)
	}
}

#[inline]
pub fn uname() -> Uname<ArrayLinuxUTSName> {
	stack_uname()
}

#[inline(always)]
pub fn stack_uname() -> Uname<ArrayLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
#[cfg( any(test, feature = "box") )]
#[inline]
pub fn box_uname() -> Uname<BoxArrayLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "cstring")))]
#[cfg( any(test, feature = "cstring") )]
#[inline]
pub fn cstring_uname() -> Uname<CStringLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn rs_uname() -> Uname<StringLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn custom_rs_uname(
	sysname: crate::beh::def_rs_linux::String, 
	nodename: crate::beh::def_rs_linux::String,
	release: crate::beh::def_rs_linux::String,
	version: crate::beh::def_rs_linux::String,
	machine: crate::beh::def_rs_linux::String,
	domainname: crate::beh::def_rs_linux::String,
) -> Uname<StringLinuxUTSName> {
	Uname::from(crate::beh::def_rs_linux::StringUtsname::new(
		sysname,
		nodename,
		release,
		version,
		machine,
		domainname,
	))
}

#[cfg_attr(docsrs, doc(cfg(feature = "cstring")))]
#[cfg( any(test, feature = "cstring") )]
#[inline]
pub fn custom_cstring_uname(
	sysname: crate::beh::def_cstring_linux::CString, 
	nodename: crate::beh::def_cstring_linux::CString,
	release: crate::beh::def_cstring_linux::CString,
	version: crate::beh::def_cstring_linux::CString,
	machine: crate::beh::def_cstring_linux::CString,
	domainname: crate::beh::def_cstring_linux::CString,
) -> Uname<CStringLinuxUTSName> {
	Uname::from(crate::beh::def_cstring_linux::CStringUtsname::new(
		sysname,
		nodename,
		release,
		version,
		machine,
		domainname,
	))
}
