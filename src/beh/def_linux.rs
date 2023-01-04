
//! Specially for dealing with raw data `libc::uname`.

extern crate libc;

use core::hash::Hash;
use core::num::NonZeroI32;
use core::ops::Deref;
use core::ops::DerefMut;
use libc::c_char;
use crate::core::UnameErr;
use crate::make_uname_data;
use ::core::ffi::CStr;
use crate::Uname;
use crate::core::GetUname;
use crate::core::AsPtrUname;

pub (crate) const CRAW_ARRAY_LINUX_UTSNAME_COUNT: usize = 65;
/// The data type used in `libc::utsname`.
pub type CRawArrayLinuxUTSName = [c_char; CRAW_ARRAY_LINUX_UTSNAME_COUNT];
/// A dimensionless data type used in `libc::utsname`.
pub type RawArrayLinuxUTSNameType = [c_char];

make_uname_data! {
	#!for_enum: /// The `libc::utsname` structure as it is present in 
	#!for_enum: /// libc without conversions. Only `[u8]`, `CStr` are supported on output. 
	#!for_enum: /// The `get` functions only execute the short `CStr::new`.
	#!for_enum: ///
	#!for_enum: /// It is recommended to use slow_write or slow_display for safe output, or 
	#!for_enum: /// use a different behavior for `uname`.
	#[pub] ArrayLinuxUTSName(#for_new(unsafe): __ArrayLinuxUTSName {
		utsname: libc::utsname,
		
		len_sysname: u8,
		len_nodename: u8,
		len_release: u8,
		len_version: u8,
		len_machine: u8,
		len_domainname: u8,
	}) {
		#[inline(always)] build_empty_data|| (__ArrayLinuxUTSName {
			utsname: unsafe { core::mem::zeroed() },
			
			len_sysname: 0,
			len_nodename: 0,
			len_release: 0,
			len_version: 0,
			len_machine: 0,
			len_domainname: 0,
		}),
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
			
			return Self::from_data(data);
		},
		#[inline(always)] from_data|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as GetUname<[u8]>>::get_sysname(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_nodename(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_release(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_version(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_machine(data), state);
			Hash::hash(<Self as GetUname<[u8]>>::get_domainname(data), state);
		},
		
		impl AsPtrUname<CRawArrayLinuxUTSName> for #self {
			#[inline(always)] sysname	|data| (&data.utsname.sysname as _),
			#[inline(always)] nodename	|data| (&data.utsname.nodename as _),
			#[inline(always)] release	|data| (&data.utsname.release as _),
			#[inline(always)] version	|data| (&data.utsname.version as _),
			#[inline(always)] machine	|data| (&data.utsname.machine as _),
			#[inline(always)] domainname	|data| (&data.utsname.domainname as _),
		}
		
		impl AsPtrUname<RawArrayLinuxUTSNameType> for #self {
			#[inline(always)] sysname	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_sysname(data) as _),
			#[inline(always)] nodename	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_nodename(data) as _),
			#[inline(always)] release	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_release(data) as _),
			#[inline(always)] version	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_version(data) as _),
			#[inline(always)] machine	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_machine(data) as _),
			#[inline(always)] domainname	|data| (<Self as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_domainname(data) as _),
		}
		
		impl GetUname<CStr> for #self {
			#[inline] sysname			|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_sysname(data) as _, 
				data.len_sysname as usize +1
			)) } ),
			#[inline] nodename			|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_nodename(data) as _, 
				data.len_nodename as usize +1
			)) } ),
			#[inline] release			|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_release(data) as _, 
				data.len_release as usize +1
			)) } ),
			#[inline] version			|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_version(data) as _, 
				data.len_version as usize +1
			)) } ),
			#[inline] machine			|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_machine(data) as _, 
				data.len_machine as usize +1
			)) } ),
			#[inline] domainname		|data| (unsafe { CStr::from_bytes_with_nul_unchecked(core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_domainname(data) as _, 
				data.len_domainname as usize +1
			)) } ),
		}
		
		impl GetUname<[u8]> for #self {
			#[inline] sysname			|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_sysname(data) as _, 
				data.len_sysname as _
			) } ),
			#[inline] nodename			|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_nodename(data) as _, 
				data.len_nodename as _
			) } ),
			#[inline] release			|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_release(data) as _, 
				data.len_release as _
			) } ),
			#[inline] version			|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_version(data) as _, 
				data.len_version as _
			) } ),
			#[inline] machine			|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_machine(data) as _, 
				data.len_machine as _
			) } ),
			#[inline] domainname		|data| (unsafe { core::slice::from_raw_parts(
				<ArrayLinuxUTSName as AsPtrUname<CRawArrayLinuxUTSName>>::as_ptr_domainname(data) as _, 
				data.len_domainname as _
			) } ),
		}
	};
}

impl Deref for __ArrayLinuxUTSName {
	type Target = libc::utsname;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.utsname
	}
}

impl DerefMut for __ArrayLinuxUTSName {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.utsname
	}
}
