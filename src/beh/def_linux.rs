
extern crate libc;

use core::hash::Hash;
use core::num::NonZeroI32;
use libc::c_char;
use crate::core::UnameErr;
use crate::make_uname_data;
use ::core::ffi::CStr;
use crate::Uname;
use crate::core::AsUname;
use crate::core::AsPtrUname;

pub type RawLinuxUTSNameType = [c_char; 65];
pub type LinuxUTSNameType = [c_char];

make_uname_data! {
	#[pub] ArrayLinuxUTSName(libc::utsname) {
		#[inline(always)] empty_data|| (unsafe { core::mem::zeroed() }),
		get_current|ok, err| {
			let mut utsname = Self::empty_data();
			match unsafe { libc::uname(&mut utsname as _) } {
				0 => ok(Self::from_data(utsname)),
				e => err(UnameErr::LibcErr(unsafe {
					NonZeroI32::new_unchecked(e)
				})),
			}
		},
		get_current_or_empty|| {
			let mut utsname = Self::empty_data();
			let _ignore_decode = unsafe { libc::uname(&mut utsname as _) };
			
			Self::from_data(utsname)
		},
		#[inline(always)] from_data|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as AsUname<[u8]>>::as_sysname(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_nodename(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_release(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_version(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_machine(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_domainname(data), state);
		},
		
		impl AsPtrUname<RawLinuxUTSNameType> for #self {
			#[inline(always)] sysname	|data| (&data.sysname as _),
			#[inline(always)] nodename	|data| (&data.nodename as _),
			#[inline(always)] release	|data| (&data.release as _),
			#[inline(always)] version	|data| (&data.version as _),
			#[inline(always)] machine	|data| (&data.machine as _),
			#[inline(always)] domainname	|data| (&data.domainname as _),
		}
		
		impl AsPtrUname<LinuxUTSNameType> for #self {
			#[inline(always)] sysname	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_sysname(data) as _),
			#[inline(always)] nodename	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_nodename(data) as _),
			#[inline(always)] release	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_release(data) as _),
			#[inline(always)] version	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_version(data) as _),
			#[inline(always)] machine	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_machine(data) as _),
			#[inline(always)] domainname	|data| (<Self as AsPtrUname<RawLinuxUTSNameType>>::as_ptr_domainname(data) as _),
		}
		
		impl AsUname<CStr> for #self {
			#[inline] sysname			|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_sysname(data) as _) }),
			#[inline] nodename			|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_nodename(data) as _) }),
			#[inline] release			|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_release(data) as _) }),
			#[inline] version			|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_version(data) as _) }),
			#[inline] machine			|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_machine(data) as _) }),
			#[inline] domainname		|data| (unsafe { CStr::from_ptr(<ArrayLinuxUTSName as AsPtrUname<LinuxUTSNameType>>::as_ptr_domainname(data) as _) }),
		}
		
		impl AsUname<[u8]> for #self {
			#[inline] sysname			|data| (<Self as AsUname<CStr>>::as_sysname(data).to_bytes()),
			#[inline] nodename			|data| (<Self as AsUname<CStr>>::as_nodename(data).to_bytes()),
			#[inline] release			|data| (<Self as AsUname<CStr>>::as_release(data).to_bytes()),
			#[inline] version			|data| (<Self as AsUname<CStr>>::as_version(data).to_bytes()),
			#[inline] machine			|data| (<Self as AsUname<CStr>>::as_machine(data).to_bytes()),
			#[inline] domainname		|data| (<Self as AsUname<CStr>>::as_domainname(data).to_bytes()),
		}
	};
}
