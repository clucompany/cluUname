
extern crate libc;
extern crate alloc;

use core::ffi::CStr;
use core::hash::Hash;
use crate::make_uname_data;
use crate::Uname;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::core::AsUname;
pub use alloc::ffi::CString;

make_uname_data! {
	#[pub] CStringLinuxUTSName( CStringUtsname {
		sysname: CString, 
		nodename: CString,
		release: CString,
		version: CString,
		machine: CString,
		domainname: CString,
	} ) {
		empty_data|| {
			let empty_cstring = unsafe {
				CString::from_vec_with_nul_unchecked(b"\0".to_vec())
			};
			CStringUtsname {
				sysname: empty_cstring.clone(),
				nodename: empty_cstring.clone(),
				release: empty_cstring.clone(),
				version: empty_cstring.clone(),
				machine: empty_cstring.clone(),
				domainname: empty_cstring,
			}
		},
		get_current|ok, err|	{
			Uname::<ArrayLinuxUTSName>::get_current_fn(
				|mut cstrcurrent| {
					let sysname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_mut_data())
						.into();
					let nodename = <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_mut_data())
						.into();
					let release = <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_mut_data())
						.into();
					let version = <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_mut_data())
						.into();
					let machine = <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_mut_data())
						.into();
					let domainname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_mut_data())
						.into();
					
					let data = Self::from_data(CStringUtsname {
						sysname,
						nodename,
						release,
						version,
						machine,
						domainname
					});
					
					ok(data)
				},
				|e| err(e)
			)
		},
		get_current_or_empty||	{
			let mut cstrcurrent: Uname<ArrayLinuxUTSName> = Uname::get_current_or_empty();
			
			let sysname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_mut_data())
				.into();
			let nodename = <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_mut_data())
				.into();
			let release = <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_mut_data())
				.into();
			let version = <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_mut_data())
				.into();
			let machine = <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_mut_data())
				.into();
			let domainname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_mut_data())
				.into();
			
			let data = CStringUtsname {
				sysname,
				nodename,
				release,
				version,
				machine,
				domainname
			};
				
			Self::from_data(data)
		},
		#[inline(always)] from_data	|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as AsUname<CString>>::as_sysname(data), state);
			Hash::hash(<Self as AsUname<CString>>::as_nodename(data), state);
			Hash::hash(<Self as AsUname<CString>>::as_release(data), state);
			Hash::hash(<Self as AsUname<CString>>::as_version(data), state);
			Hash::hash(<Self as AsUname<CString>>::as_machine(data), state);
			Hash::hash(<Self as AsUname<CString>>::as_domainname(data), state);
		},
		
		impl AsPtrUname<i8> for #self {
			#[inline(always)] sysname	|data| (data.sysname.as_ptr()),
			#[inline(always)] nodename	|data| (data.nodename.as_ptr()),
			#[inline(always)] release	|data| (data.release.as_ptr()),
			#[inline(always)] version	|data| (data.version.as_ptr()),
			#[inline(always)] machine	|data| (data.machine.as_ptr()),
			#[inline(always)] domainname	|data| (data.domainname.as_ptr()),
		}
		
		impl AsUname<CStr> for #self {
			#[inline(always)] sysname	|data| (&data.sysname),
			#[inline(always)] nodename	|data| (&data.nodename),
			#[inline(always)] release	|data| (&data.release),
			#[inline(always)] version	|data| (&data.version),
			#[inline(always)] machine	|data| (&data.machine),
			#[inline(always)] domainname	|data| (&data.domainname),
		}
		
		impl AsUname<CString> for #self {
			#[inline(always)] sysname	|data| (&data.sysname),
			#[inline(always)] nodename	|data| (&data.nodename),
			#[inline(always)] release	|data| (&data.release),
			#[inline(always)] version	|data| (&data.version),
			#[inline(always)] machine	|data| (&data.machine),
			#[inline(always)] domainname	|data| (&data.domainname),
		}
		
		impl AsUname<[u8]> for #self {
			#[inline(always)] sysname	|data| (data.sysname.as_bytes()),
			#[inline(always)] nodename	|data| (data.nodename.as_bytes()),
			#[inline(always)] release	|data| (data.release.as_bytes()),
			#[inline(always)] version	|data| (data.version.as_bytes()),
			#[inline(always)] machine	|data| (data.machine.as_bytes()),
			#[inline(always)] domainname	|data| (data.domainname.as_bytes()),
		}
	};
}
