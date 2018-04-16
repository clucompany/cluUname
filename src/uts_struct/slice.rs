

use std::ffi::CStr;
use UtsName;
use std::fmt;


#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UtsNameSlice<'a> {
	sysname:	&'a CStr,
	nodename:	&'a CStr,
	release:	&'a CStr,
	version:	&'a CStr,
	machine:	&'a CStr,
	domainname:	&'a CStr,
}

impl<'a> UtsNameSlice<'a> {
	#[inline]
	pub fn new(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> UtsNameSlice<'a> {
		UtsNameSlice {
			sysname: a1,
			nodename: a2,
			release: a3,
			version: a4,
			machine: a5,
			domainname: a6,
		}
	}
}


impl<'a> UtsName for UtsNameSlice<'a> {
	#[inline]
	fn as_sysname(&self) -> &CStr {
		self.sysname
	}
	#[inline]
	fn as_nodename(&self) -> &CStr {
		self.nodename
	}
	#[inline]
	fn as_release(&self) -> &CStr {
		self.release
	}
	#[inline]
	fn as_version(&self) -> &CStr {
		self.version
	}
	#[inline]
	fn as_machine(&self) -> &CStr {
		self.machine
	}
	#[inline]
	fn as_domainname(&self) -> &CStr {
		self.domainname
	}
}

impl<'a> fmt::Display for UtsNameSlice<'a> {
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


impl<'a> From< (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) > for UtsNameSlice<'a> {
	#[inline]
	fn from(uts: (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr)) -> UtsNameSlice<'a> {
		UtsNameSlice::new(uts.0, uts.1, uts.2, uts.3, uts.4, uts.5)
	}
}

