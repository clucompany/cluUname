
extern crate libc;
extern crate alloc;

use crate::core::AsPtrUname;
use crate::core::AsUname;
use crate::Uname;
use crate::beh::def_linux::LinuxUTSNameType;
use crate::beh::def_linux::RawLinuxUTSNameType;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::core::UnameErr;
use crate::make_uname_data;
use alloc::boxed::Box;
use core::ffi::CStr;
use core::num::NonZeroI32;
use ::core::ops::DerefMut;

make_uname_data! {
	#[pub] BoxArrayLinuxUTSName(Box<libc::utsname>) {
		// TODO, EXP STAB use of unstable library feature 'new_uninit'/'new_zeroed'
		// see issue #6329
		#[inline(always)] empty_data|| (Box::new(unsafe { core::mem::zeroed() })),
		get_current|ok, err| {
			let mut utsname = Self::empty_data();
			
			match unsafe {
				libc::uname(utsname.deref_mut() as &mut libc::utsname as _)
			} {
				0 => ok(Self::from_data(utsname)),
				e => err(UnameErr::LibcErr(unsafe {
					NonZeroI32::new_unchecked(e)
				})),
			}
		},
		#overload:
		get_current_or_empty|| {
			let mut utsname = Self::empty_data();
			
			let _ignore_decode = unsafe {
				libc::uname(utsname.deref_mut() as &mut libc::utsname as _)
			};
			Self::from_data(utsname)
		},
		
		#[inline(always)] from_data|data| (Uname::from(data)),
		#[inline(always)] hash_data|data, state| (ArrayLinuxUTSName::hash_data(data, state)),
		
		impl AsPtrUname<RawLinuxUTSNameType> for #self { #ref(ArrayLinuxUTSName) => AsPtrUname<RawLinuxUTSNameType>; }
		impl AsPtrUname<LinuxUTSNameType> for #self { #ref(ArrayLinuxUTSName) => AsPtrUname<LinuxUTSNameType>; }
		
		impl AsUname<CStr> for #self { #ref(ArrayLinuxUTSName) => AsUname<CStr>; }
		impl AsUname<[u8]> for #self { #ref(ArrayLinuxUTSName)=> AsUname<[u8]>; }
	};
}