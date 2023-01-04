
//! Specially for dealing with raw data `libc::uname`. (box version).

extern crate libc;
extern crate alloc;

use crate::beh::def_linux::__ArrayLinuxUTSName;
use crate::core::AsPtrUname;
use crate::core::GetUname;
use crate::Uname;
use crate::beh::def_linux::RawArrayLinuxUTSNameType;
use crate::beh::def_linux::CRawArrayLinuxUTSName;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::core::UnameErr;
use crate::make_uname_data;
use alloc::boxed::Box;
use core::ffi::CStr;
use core::num::NonZeroI32;

make_uname_data! {
	#!for_enum: /// Alternate version of `ArrayLinuxUTSName`.
	#!for_enum: /// The difference is in the data used, an allocator is required here.
	#!for_enum: ///
	#!for_enum: /// Optimized only for moving data and reducing stack sizes, nothing more.
	#[pub] BoxArrayLinuxUTSName(Box<__ArrayLinuxUTSName>) {
		// TODO, EXP STAB use of unstable library feature 'new_uninit'/'new_zeroed'
		// see issue #6329
		#[inline(always)] build_empty_data|| (Box::new(unsafe { core::mem::zeroed() })),
		get_current|ok, err| {
			let mut data = Self::build_empty_data();
			
			match unsafe { libc::uname(&mut data.utsname as _) } {
				0 => {
					data.len_sysname = unsafe { CStr::from_ptr(data.utsname.sysname.as_ptr()) }.to_bytes().len() as _;
					data.len_nodename = unsafe { CStr::from_ptr(data.utsname.nodename.as_ptr()) }.to_bytes().len() as _;
					data.len_release = unsafe { CStr::from_ptr(data.utsname.release.as_ptr()) }.to_bytes().len() as _;
					data.len_version = unsafe { CStr::from_ptr(data.utsname.version.as_ptr()) }.to_bytes().len() as _;
					data.len_machine = unsafe { CStr::from_ptr(data.utsname.machine.as_ptr()) }.to_bytes().len() as _;
					data.len_domainname = unsafe { CStr::from_ptr(data.utsname.domainname.as_ptr()) }.to_bytes().len() as _;
					
					ok(Self::from_data(data))
				},
				e => err(UnameErr::LibcErr(unsafe {
					NonZeroI32::new_unchecked(e)
				})),
			}
		},
		#overload:
		get_current_or_empty|| {
			let mut data = Self::build_empty_data();
			
			match unsafe { libc::uname(&mut data.utsname as _) } {
				0 => {
					data.len_sysname = unsafe { CStr::from_ptr(data.utsname.sysname.as_ptr()) }.to_bytes().len() as _;
					data.len_nodename = unsafe { CStr::from_ptr(data.utsname.nodename.as_ptr()) }.to_bytes().len() as _;
					data.len_release = unsafe { CStr::from_ptr(data.utsname.release.as_ptr()) }.to_bytes().len() as _;
					data.len_version = unsafe { CStr::from_ptr(data.utsname.version.as_ptr()) }.to_bytes().len() as _;
					data.len_machine = unsafe { CStr::from_ptr(data.utsname.machine.as_ptr()) }.to_bytes().len() as _;
					data.len_domainname = unsafe { CStr::from_ptr(data.utsname.domainname.as_ptr()) }.to_bytes().len() as _;
				},
				_e => {},
			}
			
			Self::from_data(data)
		},
		
		#[inline(always)] from_data|data| (Uname::from(data)),
		#[inline(always)] hash_data|data, state| (ArrayLinuxUTSName::hash_data(data, state)),
		
		impl AsPtrUname<CRawArrayLinuxUTSName> for #self { #ref(ArrayLinuxUTSName) => AsPtrUname<CRawArrayLinuxUTSName>; }
		impl AsPtrUname<RawArrayLinuxUTSNameType> for #self { #ref(ArrayLinuxUTSName) => AsPtrUname<RawArrayLinuxUTSNameType>; }
		
		impl GetUname<CStr> for #self { #ref(ArrayLinuxUTSName) => GetUname<CStr>; }
		impl GetUname<[u8]> for #self { #ref(ArrayLinuxUTSName) => GetUname<[u8]>; }
	};
}