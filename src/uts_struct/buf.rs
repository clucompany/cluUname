
extern crate libc;

use std::ffi::{CString, CStr};
use std::fmt;
use UtsName;
use std::os::raw::c_char;
use std::mem;


#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UtsNameBuf {
	sysname:	CString,
	nodename:	CString,
	release:	CString,
	version:	CString,
	machine:	CString,
	domainname:	CString,
}

impl UtsNameBuf {
	#[inline]
	pub fn new(a1: CString, a2: CString, a3: CString, a4: CString, a5: CString, a6: CString) -> UtsNameBuf {
		UtsNameBuf {
			sysname: a1,
			nodename: a2,
			release: a3,
			version: a4,
			machine: a5,
			domainname: a6,
		}
	}
	
	pub fn this_machine() -> Result<Self, i32> {
		let mut utsname: libc::utsname = unsafe { mem::uninitialized() };
		
		match unsafe { libc::uname(&mut utsname) } {
			0 => Ok(
				Self::from(utsname)
			),
			result => Err(result),
		}
	}
}

impl UtsName for UtsNameBuf {
	#[inline]
	fn as_sysname(&self) -> &CStr {
		&self.sysname
	}
	#[inline]
	fn as_nodename(&self) -> &CStr {
		&self.nodename
	}
	#[inline]
	fn as_release(&self) -> &CStr {
		&self.release
	}
	#[inline]
	fn as_version(&self) -> &CStr {
		&self.version
	}
	#[inline]
	fn as_machine(&self) -> &CStr {
		&self.machine
	}
	#[inline]
	fn as_domainname(&self) -> &CStr {
		&self.domainname
	}
}

impl fmt::Display for UtsNameBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let sysname = self.as_sysname();
		let nodename = self.as_nodename();
		let release = self.as_release();
		let version = self.as_version();
		let machine = self.as_machine();
		let domainname = self.as_domainname();
		
		write!(f, "{:?} {:?} {:?} {:?} {:?} {:?}", sysname, nodename, release, version, machine, domainname)
	}
}



impl From< libc::utsname > for UtsNameBuf {
	fn from(uts: libc::utsname) -> UtsNameBuf {
		let sysname = Box::new(uts.sysname);
		let nodename = Box::new(uts.nodename);
		let release = Box::new(uts.release);
		let version = Box::new(uts.version);
		let machine = Box::new(uts.machine);
		let domainname = Box::new(uts.domainname);
		
		
		let result = unsafe { UtsNameBuf {
			sysname:	CString::from_raw(sysname.as_ptr() as *mut c_char),
			nodename:	CString::from_raw(nodename.as_ptr() as *mut c_char),
			release:	CString::from_raw(release.as_ptr() as *mut c_char),
			version:	CString::from_raw(version.as_ptr() as *mut c_char),
			machine:	CString::from_raw(machine.as_ptr() as *mut c_char),
			domainname:	CString::from_raw(domainname.as_ptr() as *mut c_char),
		}};
		
		mem::forget(sysname);
		mem::forget(nodename);
		mem::forget(release);
		mem::forget(version);
		mem::forget(machine);
		mem::forget(domainname);
		
		result
	}
}

impl From< (CString, CString, CString, CString, CString, CString) > for UtsNameBuf {
	#[inline]
	fn from(uts: (CString, CString, CString, CString, CString, CString)) -> UtsNameBuf {
		UtsNameBuf::new(uts.0, uts.1, uts.2, uts.3, uts.4, uts.5)
	}
}








