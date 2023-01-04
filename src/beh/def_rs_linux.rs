
//! Specially for handling `libc::uname` parsed data specific to `UTF-8`.

extern crate libc;
extern crate alloc;

use core::hash::Hash;
use core::ffi::CStr;
use alloc::borrow::Cow;
use alloc::ffi::CString;
use alloc::string::ToString;
pub use alloc::string::String;
use crate::beh::def_linux::CRawArrayLinuxUTSName;
use crate::beh::def_linux::CRAW_ARRAY_LINUX_UTSNAME_COUNT;
use crate::core::GetUname;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::Uname;
use crate::core::UnameBeh;
use crate::make_uname_data;

make_uname_data! {
	#[pub] StringLinuxUTSName( StringUtsname {
		sysname: String, 
		nodename: String,
		release: String,
		version: String,
		machine: String,
		domainname: String,
	} ) {
		build_empty_data|| {
			let empty_string = String::new();
			
			StringUtsname {
				sysname: empty_string.clone(),
				nodename: empty_string.clone(),
				release: empty_string.clone(),
				version: empty_string.clone(),
				machine: empty_string.clone(),
				domainname: empty_string,
			}
		},
		get_current|ok, err| {
			Uname::<ArrayLinuxUTSName>::get_current_fn(
				|cstrcurrent| {
					let data = cstrcurrent.as_data();
					
					let sysname = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_sysname(data)
					).into_owned();
					let nodename = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_nodename(data)
					).into_owned();
					let release = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_release(data)
					).into_owned();
					let version = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_version(data)
					).into_owned();
					let machine = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_machine(data)
					).into_owned();
					let domainname = String::from_utf8_lossy(
						<ArrayLinuxUTSName as GetUname::<[u8]>>::get_domainname(data)
					).into_owned();
					
					ok(Self::from_data(StringUtsname {
						sysname,
						nodename,
						release,
						version,
						machine,
						domainname
					}))
				},
				|e| err(e)
			)
		},
		#[inline(always)] from_data	|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as GetUname<String>>::get_sysname(data), state);
			Hash::hash(<Self as GetUname<String>>::get_nodename(data), state);
			Hash::hash(<Self as GetUname<String>>::get_release(data), state);
			Hash::hash(<Self as GetUname<String>>::get_version(data), state);
			Hash::hash(<Self as GetUname<String>>::get_machine(data), state);
			Hash::hash(<Self as GetUname<String>>::get_domainname(data), state);
		},
		
		impl AsPtrUname<u8> for #self {
			#[inline(always)] sysname	|data| (data.sysname.as_ptr()),
			#[inline(always)] nodename	|data| (data.nodename.as_ptr()),
			#[inline(always)] release	|data| (data.release.as_ptr()),
			#[inline(always)] version	|data| (data.version.as_ptr()),
			#[inline(always)] machine	|data| (data.machine.as_ptr()),
			#[inline(always)] domainname	|data| (data.domainname.as_ptr()),
		}
		
		impl GetUname<str> for #self {
			#[inline(always)] sysname	|data| (&data.sysname),
			#[inline(always)] nodename	|data| (&data.nodename),
			#[inline(always)] release	|data| (&data.release),
			#[inline(always)] version	|data| (&data.version),
			#[inline(always)] machine	|data| (&data.machine),
			#[inline(always)] domainname	|data| (&data.domainname),
		}
		
		impl GetUname<String> for #self {
			#[inline(always)] sysname	|data| (&data.sysname),
			#[inline(always)] nodename	|data| (&data.nodename),
			#[inline(always)] release	|data| (&data.release),
			#[inline(always)] version	|data| (&data.version),
			#[inline(always)] machine	|data| (&data.machine),
			#[inline(always)] domainname	|data| (&data.domainname),
		}
		
		impl GetUname<[u8]> for #self {
			#[inline(always)] sysname	|data| (data.sysname.as_bytes()),
			#[inline(always)] nodename	|data| (data.nodename.as_bytes()),
			#[inline(always)] release	|data| (data.release.as_bytes()),
			#[inline(always)] version	|data| (data.version.as_bytes()),
			#[inline(always)] machine	|data| (data.machine.as_bytes()),
			#[inline(always)] domainname	|data| (data.domainname.as_bytes()),
		}
	};
	
	#!for_enum: /// The optimal version of `uname` to use inside a `rust` safe.
	#!for_enum: /// Only `[u8]`, `str` are supported on output. 
	#!for_enum: ///
	#!for_enum: /// Each element of `uname` is checked for compatibility with 
	#!for_enum: /// `UTF-8` when creating `uname`. If the `uname` element is to be escaped, 
	#!for_enum: /// the allocator is used; if not, a reference is used. Each `uname` element 
	#!for_enum: /// is an alternative to Cow, but has no lifetime on its own.
	#!for_enum: ///
	#!for_enum: /// Recommended only if you are using `UTF-8` `uname` safe output, if you don't need it, use the original `uname`.
	#[pub] CowStrLinuxUTSName( #for_new(unsafe): CowStrUtsname { // `#for_new(unsafe)` makes the `new` function unsafe, since you must read the comment for every `uname` field that you can output so as not to break the output of `Uname`.
		utsname: Uname<ArrayLinuxUTSName>,
		
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		sysname: Option<String>,
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		nodename: Option<String>, 
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		release: Option<String>,
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		version: Option<String>,
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		machine: Option<String>,
		// (ATTENTION!! if escaping is required, then this is the Some(String) 
		// field with escaping, if escaping is not required, then None here and the 
		// data reference is taken from utsname)
		domainname: Option<String>,
	} ) {
		build_empty_data|| {
			CowStrUtsname {
				utsname: Uname::empty(),
				
				sysname: None,
				nodename: None,
				release: None,
				version: None,
				machine: None,
				domainname: None,
			}
		},
		get_current|ok, err| {
			Uname::<ArrayLinuxUTSName>::get_current_fn(
				|cstrcurrent| {
					let data = cstrcurrent.as_data();
					/*
						The basic idea is to pass all data through `lossy`, 
						if the string was escaped then use it, if the string was 
						not escaped and it was returned as is then use None and refer to 
						`libc::utsname`.
						
						"Cow" was not used due to the reference of the leading structure to itself.
					*/
					
					let sysname = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_sysname(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let nodename = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_nodename(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let release = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_release(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let version = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_version(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let machine = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_machine(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let domainname = match <ArrayLinuxUTSName as GetUname::<CStr>>::get_domainname(data).to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					
					ok(Self::from_data(CowStrUtsname {
						utsname: cstrcurrent,
						
						sysname,
						nodename,
						release,
						version,
						machine,
						domainname,
					}))
				},
				|e| err(e)
			)
		},
		#[inline(always)] from_data	|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as GetUname<[u8]>>::get_sysname(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_nodename(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_release(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_version(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_machine(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_domainname(data), state);
		},
		
		impl AsPtrUname<u8> for #self {
			/*
				This is optional, but allows you to refer to specific data being used.
			*/
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_sysname().as_ptr(),
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_nodename().as_ptr(),
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_release().as_ptr(),
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_version().as_ptr(),
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_machine().as_ptr(),
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.get_domainname().as_ptr(),
			}),
		}
		
		impl GetUname<str> for #self {
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_sysname()) },
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_nodename()) },
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_release()) },
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_version()) },
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_machine()) },
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_str(),
				// This is safe because the line was checked for safety when `uname` was created.
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.get_domainname()) },
			}),
		}
		
		impl GetUname<[u8]> for #self {
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_sysname(),
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_nodename(),
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_release(),
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_version(),
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_machine(),
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.get_domainname(),
			}),
		}
	};
}


pub trait CustomUnameDataBuilder where Self: Sized {
	fn make_ownedstring_or_writearray<'a, 'n>(
		self, 
		warray: &'a mut CRawArrayLinuxUTSName,
	) -> (Option<String>, u8);
}

impl CustomUnameDataBuilder for String {
	#[inline]
	fn make_ownedstring_or_writearray(self, _warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		(Some(self), 0)
	}
}

impl<'a> CustomUnameDataBuilder for Cow<'a, str> {
	#[inline]
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		match self {
			Cow::Owned(a) => CustomUnameDataBuilder::make_ownedstring_or_writearray(a, warray),
			Cow::Borrowed(a) => CustomUnameDataBuilder::make_ownedstring_or_writearray(a, warray),
		}
	}
}

impl<'a> CustomUnameDataBuilder for CRawArrayLinuxUTSName {
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		// ignore check len
		let sself_array: &[u8] = unsafe {
			&*(&self as &CRawArrayLinuxUTSName as *const [i8] as *const _)
		};
		if let Cow::Owned(str) = String::from_utf8_lossy(sself_array) {
			return (Some(str), 0);
		}
		drop(sself_array);
		warray.copy_from_slice(&self as &CRawArrayLinuxUTSName);
		
		(None, self.len() as _)
	}
}

impl<'a> CustomUnameDataBuilder for &'a str {
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		let len = self.len();
		if len > CRAW_ARRAY_LINUX_UTSNAME_COUNT {
			return (Some(self.to_string()), 0);
		}
		// ignore check lossy
		
		let array = self.as_bytes();
		let sself_array: &[i8] = unsafe {
			&*(array as &[u8] as *const [u8] as *const _)
		};
		
		debug_assert_eq!(warray.get(..len).is_some(), true);
		unsafe {
			warray.get_unchecked_mut(..len)
		}.copy_from_slice(sself_array);
		
		(None, len as _)
	}
}

impl<'a> CustomUnameDataBuilder for &'a CStr {
	#[inline]
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		let array = self.to_bytes();
		
		CustomUnameDataBuilder::make_ownedstring_or_writearray(
			array, 
			warray,
		)
	}
}

impl<'a> CustomUnameDataBuilder for CString {
	#[inline]
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		let array = self.to_bytes();
		
		CustomUnameDataBuilder::make_ownedstring_or_writearray(
			array, 
			warray, 
		)
	}
}

impl<'a> CustomUnameDataBuilder for &'a [u8] {
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		let len = self.len();
		if len > CRAW_ARRAY_LINUX_UTSNAME_COUNT {
			return (Some(String::from_utf8_lossy(self).into_owned()), 0);
		}
		if let Cow::Owned(str) = String::from_utf8_lossy(self) {
			return (Some(str), 0);
		}
		let sself_array: &[i8] = unsafe {
			&*(self as &[u8] as *const [u8] as *const _)
		};
		
		debug_assert_eq!(warray.get(..len).is_some(), true);
		unsafe {
			warray.get_unchecked_mut(..len)
		}.copy_from_slice(sself_array);
		
		(None, len as _)
	}
}

impl<'a> CustomUnameDataBuilder for &'a [i8] {
	#[inline]
	fn make_ownedstring_or_writearray(self, warray: &mut CRawArrayLinuxUTSName) -> (Option<String>, u8) {
		let sself_array: &[u8] = unsafe {
			&*(self as &[i8] as *const [i8] as *const _)
		};
		
		CustomUnameDataBuilder::make_ownedstring_or_writearray(
			sself_array, 
			warray, 
		)
	}
}

pub fn custom_uname(
	sysname: impl CustomUnameDataBuilder, 
	nodename: impl CustomUnameDataBuilder, 
	release: impl CustomUnameDataBuilder, 
	version: impl CustomUnameDataBuilder, 
	machine: impl CustomUnameDataBuilder, 
	domainname: impl CustomUnameDataBuilder, 
) -> Uname<CowStrLinuxUTSName> {
	let mut utsname = ArrayLinuxUTSName::build_empty_data();
	
	// TODO
	let (sysname, len) = sysname.make_ownedstring_or_writearray(
		&mut utsname.sysname, 
	);
	utsname.len_sysname = len;
	
	// TODO
	let (nodename, len) = nodename.make_ownedstring_or_writearray(
		&mut utsname.nodename, 
	);
	utsname.len_nodename = len;
	
	let (release, len) = release.make_ownedstring_or_writearray(
		&mut utsname.release, 
	);
	utsname.len_release = len;
	
	let (version, len) = version.make_ownedstring_or_writearray(
		&mut utsname.version,
	);
	utsname.len_version = len;
	
	let (machine, len) = machine.make_ownedstring_or_writearray(
		&mut utsname.machine, 
	);
	utsname.len_machine = len;
	
	let (domainname, len) = domainname.make_ownedstring_or_writearray(
		&mut utsname.domainname, 
	);
	utsname.len_domainname = len;
	
	let data = crate::beh::def_rs_linux::CowStrUtsname {
		utsname: Uname::from(utsname),
		
		sysname,
		nodename,
		release,
		version,
		machine,
		domainname
	};
	Uname::from(data)
}
