
extern crate libc;
extern crate alloc;

use core::hash::Hash;
use core::ffi::CStr;
pub use alloc::string::String;
use crate::core::AsUname;
use crate::beh::def_linux::ArrayLinuxUTSName;
use crate::Uname;
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
		empty_data|| {
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
				|mut cstrcurrent| {
					let sysname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					let nodename = <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					let release = <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					let version = <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					let machine = <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					let domainname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_mut_data())
						.to_string_lossy().into_owned();
					
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
		get_current_or_empty|| {
			let mut cstrcurrent: Uname<ArrayLinuxUTSName> = Uname::get_current_or_empty();
			
			let sysname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			let nodename = <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			let release = <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			let version = <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			let machine = <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			let domainname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_mut_data())
				.to_string_lossy().into_owned();
			
			Self::from_data(StringUtsname {
				sysname,
				nodename,
				release,
				version,
				machine,
				domainname
			})
		},
		
		#[inline(always)] from_data	|data| (Uname::from(data)),
		
		hash_data|data, state| {
			Hash::hash(<Self as AsUname<String>>::as_sysname(data), state);
			Hash::hash(<Self as AsUname<String>>::as_nodename(data), state);
			Hash::hash(<Self as AsUname<String>>::as_release(data), state);
			Hash::hash(<Self as AsUname<String>>::as_version(data), state);
			Hash::hash(<Self as AsUname<String>>::as_machine(data), state);
			Hash::hash(<Self as AsUname<String>>::as_domainname(data), state);
		},
		
		impl AsPtrUname<u8> for #self {
			#[inline(always)] sysname	|data| (data.sysname.as_ptr()),
			#[inline(always)] nodename	|data| (data.nodename.as_ptr()),
			#[inline(always)] release	|data| (data.release.as_ptr()),
			#[inline(always)] version	|data| (data.version.as_ptr()),
			#[inline(always)] machine	|data| (data.machine.as_ptr()),
			#[inline(always)] domainname	|data| (data.domainname.as_ptr()),
		}
		
		impl AsUname<str> for #self {
			#[inline(always)] sysname	|data| (&data.sysname),
			#[inline(always)] nodename	|data| (&data.nodename),
			#[inline(always)] release	|data| (&data.release),
			#[inline(always)] version	|data| (&data.version),
			#[inline(always)] machine	|data| (&data.machine),
			#[inline(always)] domainname	|data| (&data.domainname),
		}
		
		impl AsUname<String> for #self {
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
