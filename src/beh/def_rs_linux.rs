
extern crate libc;
extern crate alloc;

use core::hash::Hash;
use core::ffi::CStr;
use alloc::borrow::Cow;
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
				|cstrcurrent| {
					let sysname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_data())
						.to_string_lossy().into_owned();
					let nodename = <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_data())
						.to_string_lossy().into_owned();
					let release = <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_data())
						.to_string_lossy().into_owned();
					let version = <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_data())
						.to_string_lossy().into_owned();
					let machine = <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_data())
						.to_string_lossy().into_owned();
					let domainname = <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_data())
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
	
	#[pub] CowStrLinuxUTSName( CowStrUtsname {
		utsname: Uname<ArrayLinuxUTSName>,
		
		sysname: Option<String>,
		nodename: Option<String>,
		release: Option<String>,
		version: Option<String>,
		machine: Option<String>,
		domainname: Option<String>,
	} ) {
		empty_data|| {
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
					let sysname = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_sysname(cstrcurrent.as_data())
						.to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let nodename = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_nodename(cstrcurrent.as_data())
						.to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let release = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_release(cstrcurrent.as_data())
						.to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let version = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_version(cstrcurrent.as_data())
						.to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let machine = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_machine(cstrcurrent.as_data())
						.to_string_lossy() {
						Cow::Owned(a) => Some(a),
						Cow::Borrowed(..) => None,
					};
					let domainname = match <ArrayLinuxUTSName as AsUname::<CStr>>::as_domainname(cstrcurrent.as_data())
						.to_string_lossy() {
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
			Hash::hash(<Self as AsUname<[u8]>>::as_sysname(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_nodename(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_release(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_version(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_machine(data), state);
			Hash::hash(<Self as AsUname<[u8]>>::as_domainname(data), state);
		},
		
		impl AsPtrUname<u8> for #self {
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_sysname().as_ptr(),
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_nodename().as_ptr(),
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_release().as_ptr(),
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_version().as_ptr(),
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_machine().as_ptr(),
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_ptr(),
				None => data.utsname.as_domainname().as_ptr(),
			}),
		}
		
		impl AsUname<str> for #self {
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_sysname()) },
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_nodename()) },
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_release()) },
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_version()) },
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_machine()) },
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_str(),
				None => unsafe { alloc::str::from_utf8_unchecked(data.utsname.as_domainname()) },
			}),
		}
		
		impl AsUname<[u8]> for #self {
			#[inline] sysname	|data| (match data.sysname {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_sysname(),
			}),
			#[inline] nodename	|data| (match data.nodename {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_nodename(),
			}),
			#[inline] release	|data| (match data.release {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_release(),
			}),
			#[inline] version	|data| (match data.version {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_version(),
			}),
			#[inline] machine	|data| (match data.machine {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_machine(),
			}),
			#[inline] domainname	|data| (match data.domainname {
				Some(ref a) => a.as_bytes(),
				None => data.utsname.as_domainname(),
			}),
		}
	};
}
