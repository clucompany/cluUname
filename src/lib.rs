//Copyright 2019-2023 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2023
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
#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![allow(non_snake_case)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
#[cfg( any(test, feature = "box") )]
use crate::beh::def_box_linux::BoxArrayLinuxUTSName;
#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
use crate::beh::def_rs_linux::StringLinuxUTSName;
#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
use crate::beh::def_rs_linux::CowStrLinuxUTSName;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::beh::def_linux::RawArrayLinuxUTSNameType;
use crate::beh::def_linux::CRawArrayLinuxUTSName;
use crate::core::GetUname;
use crate::core::AsPtrUname;
use crate::core::UnameBeh;
use crate::core::UnameErr;
use ::core::ffi::CStr;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::Write;
use ::core::hash::Hash;
use ::core::fmt::Debug;

/// Specifies the behavior of the `Uname` to use.
pub mod beh {
	pub mod def_linux;
	
	#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
	#[cfg( any(test, feature = "box") )]
	pub mod def_box_linux;
	
	#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
	#[cfg( any(test, feature = "rs") )]
	pub mod def_rs_linux;
}

mod macro_make;
pub mod core;

/// A `uname` struct to get the current `uname` as well as each of its fields (sysname, nodename, release, version, machine, domainname). 
///
/// Additionally implemented features `display`, `hash`, `debug`, ...
#[repr(transparent)]
pub struct Uname<D> where D: UnameBeh {
	/// The data that is stored inside the `Uname` structure.
	data: D::Data,
}

impl<D> Debug for Uname<D> where D: UnameBeh + GetUname<[u8], Data = <D as UnameBeh>::Data> {
	fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
		let data = self.as_data();
		
		f.debug_struct("Uname")
			.field("sysname", &D::get_sysname(data))
			.field("nodename", &D::get_nodename(data))
			.field("release", &D::get_release(data))
			.field("version", &D::get_version(data))
			.field("machine", &D::get_machine(data))
			.field("domainname", &D::get_domainname(data))
			.finish()
	}
}

impl<D> Hash for Uname<D> where D: UnameBeh {
	#[inline(always)]
	fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
		D::hash_data(self.as_data(), state)
	}
}

impl<D> Uname<D> where D: UnameBeh {
	/// Get the currently populated `uname` on success, 
	/// or an error on failure.
	#[inline(always)]
	pub fn get_current() -> Result<Uname<D>, UnameErr> {
		D::get_current()
	}
	
	/// Get the currently populated `uname` on success and call the `ok` 
	/// function or error and call the `err` function with an error on failure.
	#[inline(always)]
	pub fn get_current_fn<R>(ok: impl FnOnce(Uname<D>) -> R, e: impl FnOnce(UnameErr) -> R) -> R {
		D::get_current_fn(ok, e)
	}
	
	/// Get the currently populated `uname` on success, or an empty 
	/// unpopulated `uname` on failure.
	#[inline(always)]
	pub fn get_current_or_empty() -> Uname<D> {
		D::get_current_or_empty()
	}
	
	/// Create an empty `uname` with no data filled in.
	#[inline]
	pub fn empty() -> Uname<D> {
		Self::from(D::build_empty_data())
	}
	
	/// Create a `uname` structure with external arbitrary data.
	#[inline]
	pub const fn from(data: D::Data) -> Self {
		Self {
			data,
		}
	}
	
	/// Get a reference to the `uname` data.
	#[inline(always)]
	pub const fn as_data(&self) -> &D::Data {
		&self.data
	}
	
	/// Get a mut reference to the `uname` data.
	#[inline(always)]
	pub /*const*/ fn as_mut_data(&mut self) -> &mut D::Data {
		&mut self.data
	}
}

/// An internal structure to implement a safe but slow "display" 
/// 
/// (optimal for a one-time call, if you call it one or more times, 
/// consider converting the data).
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

impl<D> Uname<D> where D: UnameBeh + GetUname<[u8], Data = <D as UnameBeh>::Data> {
	/// Get the hash of the `uname` data by taking the hash from `[u8]` 
	/// and only the get functions. 
	/// 
	/// 1. Only available with the `std` flag.
	/// 2. The `Get` functions use minimal or no conversion if no conversions were 
	/// written when `uname` was created. (see your implementation of the `uname` behavior)
	#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
	#[cfg( any(test, feature = "std") )]
	pub fn get_hash_version(&self) -> u64 {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::Hasher;

		let mut s = DefaultHasher::new();
		self.hash(&mut s);
		s.finish()
	}
	
	/// Slow function to safely write `uname` data to a single line. 
	/// Slow because it takes [u8] as is (see uname behavior in use) and 
	/// checks every byte during writing to make sure the output is valid.
	/// 
	/// It is recommended to use only in case of unhandled uname behavior and only 
	/// for one call (more than one call may incur overhead than if you took a cast when creating uname).
	pub fn slow_write_to<W: Write>(&self, w: &mut W) -> ::core::fmt::Result {
		write!(
			w, "{} {} {} {} {} {}", 
			_SlowSafeDisplay(self.get_sysname()), 
			_SlowSafeDisplay(self.get_nodename()),
			_SlowSafeDisplay(self.get_release()),
			_SlowSafeDisplay(self.get_version()),
			_SlowSafeDisplay(self.get_machine()),
			_SlowSafeDisplay(self.get_domainname()),
		)
	}
	
	/// A function to get a slow `map` to safely output `uname` data on a single line. 
	/// Slow because it takes [u8] as it is (see uname behavior) and checks each byte 
	/// during writing to make sure the output is valid.
	/// 
	/// It is recommended to use only in case of unhandled uname behavior and only 
	/// for one call to display::fmt (more than one call may incur overhead than if you took a cast when creating uname).
	pub fn get_slow_display<'a>(&'a self) -> impl Display + 'a {
		struct __FullDisplay<'a, D>(&'a Uname<D>) where D: UnameBeh + GetUname<[u8], Data = <D as UnameBeh>::Data>;
		
		impl<'a, D> Display for __FullDisplay<'a, D> where D: UnameBeh + GetUname<[u8], Data = <D as UnameBeh>::Data> {
			fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
				write!(
					f, "{} {} {} {} {} {}", 
					_SlowSafeDisplay(self.0.get_sysname()), 
					_SlowSafeDisplay(self.0.get_nodename()),
					_SlowSafeDisplay(self.0.get_release()),
					_SlowSafeDisplay(self.0.get_version()),
					_SlowSafeDisplay(self.0.get_machine()),
					_SlowSafeDisplay(self.0.get_domainname()),
				)
			}
		}
		
		__FullDisplay(self)
	}
	
	/// Get a reference to the `sysname` data array.
	#[inline(always)]
	pub fn get_sysname<'a>(&'a self) -> &'a [u8] {
		D::get_sysname(&self.data)
	}
	
	/// Get a reference to the `nodename` data array.
	#[inline(always)]
	pub fn get_nodename<'a>(&'a self) -> &'a [u8] {
		D::get_nodename(&self.data)
	}
	
	/// Get a reference to the `release` data array.
	#[inline(always)]
	pub fn get_release<'a>(&'a self) -> &'a [u8] {
		D::get_release(&self.data)
	}
	
	/// Get a reference to the `version` data array.
	#[inline(always)]
	pub fn get_version<'a>(&'a self) -> &'a [u8] {
		D::get_version(&self.data)
	}
	
	/// Get a reference to the `machine` data array.
	#[inline(always)]
	pub fn get_machine<'a>(&'a self) -> &'a [u8] {
		D::get_machine(&self.data)
	}
	
	/// Get a reference to the `domainname` data array.
	#[inline(always)]
	pub fn get_domainname<'a>(&'a self) -> &'a [u8] {
		D::get_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameBeh + GetUname<CStr, Data = <D as UnameBeh>::Data> {
	/// Get a reference to the `sysname` data cstr.
	#[inline(always)]
	pub fn get_cstr_sysname<'a>(&'a self) -> &'a CStr {
		D::get_sysname(&self.data)
	}
	
	/// Get a reference to the `nodename` data cstr.
	#[inline(always)]
	pub fn get_cstr_nodename<'a>(&'a self) -> &'a CStr {
		D::get_nodename(&self.data)
	}
	
	/// Get a reference to the `release` data cstr.
	#[inline(always)]
	pub fn get_cstr_release<'a>(&'a self) -> &'a CStr {
		D::get_release(&self.data)
	}
	
	/// Get a reference to the `version` data cstr.
	#[inline(always)]
	pub fn get_cstr_version<'a>(&'a self) -> &'a CStr {
		D::get_version(&self.data)
	}
	
	/// Get a reference to the `machine` data cstr.
	#[inline(always)]
	pub fn get_cstr_machine<'a>(&'a self) -> &'a CStr {
		D::get_machine(&self.data)
	}
	
	/// Get a reference to the `domainname` data cstr.
	#[inline(always)]
	pub fn get_cstr_domainname<'a>(&'a self) -> &'a CStr {
		D::get_domainname(&self.data)
	}
}

impl<D> Display for Uname<D> where D: UnameBeh + GetUname<str, Data = <D as UnameBeh>::Data> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> ::core::fmt::Result {
		Display::fmt(&self.get_display(), f)
	}
}

impl<D> Uname<D> where D: UnameBeh + GetUname<str, Data = <D as UnameBeh>::Data> {
	/// Write the `uname` data to `fmt::Write` in single line format. 
	/// (get functions are used, so conversion is possible at the stage 
	/// of calling these functions or at the stage of creating the `uname` structure, 
	/// see the behavior you use for uname.)
	#[inline]
	pub fn write_to<W: Write>(&self, w: &mut W) -> ::core::fmt::Result {
		write!(
			w, "{} {} {} {} {} {}", 
			self.get_str_sysname(), 
			self.get_str_nodename(),
			self.get_str_release(),
			self.get_str_version(),
			self.get_str_machine(),
			self.get_str_domainname(),
		)
	}
	
	/// Get `display` of `uname` data in single line format. 
	/// (get functions are used, so conversion is possible at the stage 
	/// of calling these functions or at the stage of creating the `uname` structure, 
	/// see the behavior you use for uname.)
	pub fn get_display<'a>(&'a self) -> impl Display + 'a {
		struct __FullDisplay<'a, D>(&'a Uname<D>) where D: UnameBeh + GetUname<str, Data = <D as UnameBeh>::Data>;
		
		impl<'a, D> Display for __FullDisplay<'a, D> where D: UnameBeh + GetUname<str, Data = <D as UnameBeh>::Data> {
			#[inline]
			fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
				write!(
					f, "{} {} {} {} {} {}", 
					self.0.get_str_sysname(), 
					self.0.get_str_nodename(),
					self.0.get_str_release(),
					self.0.get_str_version(),
					self.0.get_str_machine(),
					self.0.get_str_domainname(),
				)
			}
		}
		
		__FullDisplay(self)
	}
	
	/// Get a reference to the `sysname` data str.
	#[inline(always)]
	pub fn get_str_sysname<'a>(&'a self) -> &'a str {
		D::get_sysname(&self.data)
	}
	
	/// Get a reference to the `nodename` data str.
	#[inline(always)]
	pub fn get_str_nodename<'a>(&'a self) -> &'a str {
		D::get_nodename(&self.data)
	}
	
	/// Get a reference to the `release` data str.
	#[inline(always)]
	pub fn get_str_release<'a>(&'a self) -> &'a str {
		D::get_release(&self.data)
	}
	
	/// Get a reference to the `version` data str.
	#[inline(always)]
	pub fn get_str_version<'a>(&'a self) -> &'a str {
		D::get_version(&self.data)
	}
	
	/// Get a reference to the `machine` data str.
	#[inline(always)]
	pub fn get_str_machine<'a>(&'a self) -> &'a str {
		D::get_machine(&self.data)
	}
	
	/// Get a reference to the `domainname` data str.
	#[inline(always)]
	pub fn get_str_domainname<'a>(&'a self) -> &'a str {
		D::get_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameBeh + AsPtrUname<RawArrayLinuxUTSNameType, Data = <D as UnameBeh>::Data> {
	/// Get a raw pointer to the `sysname` data.
	#[inline(always)]
	pub fn as_ptr_sysname(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_sysname(&self.data)
	}
	
	/// Get a raw pointer to the `nodename` data.
	#[inline(always)]
	pub fn as_ptr_nodename(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_nodename(&self.data)
	}
	
	/// Get a raw pointer to the `release` data.
	#[inline(always)]
	pub fn as_ptr_release(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_release(&self.data)
	}
	
	/// Get a raw pointer to the `version` data.
	#[inline(always)]
	pub fn as_ptr_version(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_version(&self.data)
	}
	
	/// Get a raw pointer to the `machine` data.
	#[inline(always)]
	pub fn as_ptr_machine(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_machine(&self.data)
	}
	
	/// Get a raw pointer to the `domainname` data.
	#[inline(always)]
	pub fn as_ptr_domainname(&self) -> *const RawArrayLinuxUTSNameType {
		D::as_ptr_domainname(&self.data)
	}
}

impl<D> Uname<D> where D: UnameBeh + AsPtrUname<CRawArrayLinuxUTSName, Data = <D as UnameBeh>::Data> {
	/// Get a raw pointer to the `sysname` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_sysname(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_sysname(&self.data)
	}
	
	/// Get a raw pointer to the `nodename` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_nodename(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_nodename(&self.data)
	}
	
	/// Get a raw pointer to the `release` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_release(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_release(&self.data)
	}
	
	/// Get a raw pointer to the `version` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_version(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_version(&self.data)
	}
	
	/// Get a raw pointer to the `machine` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_machine(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_machine(&self.data)
	}
	
	/// Get a raw pointer to the `domainname` data. (without any transformations).
	#[inline(always)]
	pub fn as_rawptr_domainname(&self) -> *const CRawArrayLinuxUTSName {
		D::as_ptr_domainname(&self.data)
	}
}

#[inline]
pub fn uname() -> Result<Uname<ArrayLinuxUTSName>, UnameErr> {
	stack_uname()
}

#[inline(always)]
pub fn stack_uname() -> Result<Uname<ArrayLinuxUTSName>, UnameErr> {
	Uname::get_current()
}

#[inline(always)]
pub fn stack_uname_or_empty() -> Uname<ArrayLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
#[cfg( any(test, feature = "box") )]
#[inline]
pub fn box_uname() -> Result<Uname<BoxArrayLinuxUTSName>, UnameErr> {
	Uname::get_current()
}

#[cfg_attr(docsrs, doc(cfg(feature = "box")))]
#[cfg( any(test, feature = "box") )]
#[inline]
pub fn box_uname_or_empty() -> Uname<BoxArrayLinuxUTSName> {
	Uname::get_current_or_empty()
}

/// Get `Uname` or return an error. 
/// `Uname` is adapted to `UTF-8` and uses `CowStr` internally.
#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn rs_uname() -> Result<Uname<CowStrLinuxUTSName>, UnameErr> {
	Uname::get_current()
}

/// Get `Uname` or return an empty `Uname`. 
/// `Uname` is adapted to `UTF-8` and uses `CowStr` internally.
#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn rs_uname_or_empty() -> Uname<CowStrLinuxUTSName> {
	Uname::get_current_or_empty()
}

#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn custom_string_uname(
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

pub trait MaybeBuildCustomUname {}

#[cfg_attr(docsrs, doc(cfg(feature = "rs")))]
#[cfg( any(test, feature = "rs") )]
#[inline]
pub fn custom_rs_uname(
	sysname: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
	nodename: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
	release: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
	version: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
	machine: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
	domainname: impl crate::beh::def_rs_linux::CustomUnameDataBuilder,
) -> Uname<CowStrLinuxUTSName> {
	crate::beh::def_rs_linux::custom_uname(
		sysname, 
		nodename, 
		release, 
		version, 
		machine, 
		domainname
	)
}
