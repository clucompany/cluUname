

use uts_struct::buf::UtsNameBuf;
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
	
	#[cfg(feature = "enable_domainname")]
	domainname:	&'a CStr,
}

impl<'a> UtsNameSlice<'a> {
	#[cfg(feature = "enable_domainname")]
	#[inline]
	pub fn new(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr, a6: &'a CStr) -> Self {
		Self {
			sysname: a1,
			nodename: a2,
			release: a3,
			version: a4,
			machine: a5,
			domainname: a6,
		}
	}
	
	#[cfg(not(feature = "enable_domainname"))]
	#[inline]
	pub fn new(a1: &'a CStr, a2: &'a CStr, a3: &'a CStr, a4: &'a CStr, a5: &'a CStr) -> Self {
		Self {
			sysname: a1,
			nodename: a2,
			release: a3,
			version: a4,
			machine: a5,
		}
	}
}


impl<'a> UtsName for UtsNameSlice<'a> {
	#[inline(always)]
	fn as_sysname(&self) -> &CStr {
		self.sysname
	}
	#[inline(always)]
	fn as_nodename(&self) -> &CStr {
		self.nodename
	}
	#[inline(always)]
	fn as_release(&self) -> &CStr {
		self.release
	}
	#[inline(always)]
	fn as_version(&self) -> &CStr {
		self.version
	}
	#[inline(always)]
	fn as_machine(&self) -> &CStr {
		self.machine
	}
	
	#[cfg(feature = "enable_domainname")]
	#[inline(always)]
	fn as_domainname(&self) -> &CStr {
		self.domainname
	}
}



impl<'a> fmt::Display for UtsNameSlice<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let sysname = self.display_sysname();
		let nodename = self.display_nodename();
		let release = self.display_release();
		let version = self.display_version();
		let machine = self.display_machine();
		
		#[cfg(feature = "enable_domainname")]
		let domainname = self.display_domainname();
		
		#[cfg(feature = "enable_domainname")]
		return write!(f, "{} {} {} {} {} {}", sysname, nodename, release, version, machine, domainname);
		
		#[cfg(not(feature = "enable_domainname"))]
		return write!(f, "{} {} {} {} {}", sysname, nodename, release, version, machine);
	}
}

#[cfg(feature = "enable_domainname")]
impl<'a> From< (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) > for UtsNameSlice<'a> {
	#[inline]
	fn from(uts: (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr)) -> UtsNameSlice<'a> {
		UtsNameSlice::new(uts.0, uts.1, uts.2, uts.3, uts.4, uts.5)
	}
}

#[cfg(not(feature = "enable_domainname"))]
impl<'a> From< (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) > for UtsNameSlice<'a> {
	#[inline]
	fn from(uts: (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr)) -> UtsNameSlice<'a> {
		UtsNameSlice::new(uts.0, uts.1, uts.2, uts.3, uts.4)
	}
}


#[cfg(feature = "enable_domainname")]
impl<'a> Into< (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) > for UtsNameSlice<'a> {
	#[inline]
	fn into(self) -> (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) {
		(self.sysname, self.nodename, self.release, self.version, self.machine, self.domainname)
	}
}

#[cfg(not(feature = "enable_domainname"))]
impl<'a> Into< (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) > for UtsNameSlice<'a> {
	#[inline]
	fn into(self) -> (&'a CStr, &'a CStr, &'a CStr, &'a CStr, &'a CStr) {
		(self.sysname, self.nodename, self.release, self.version, self.machine)
	}
}




impl<'a> From< &'a UtsNameBuf > for UtsNameSlice<'a> {
	#[inline]
	fn from(uts: &'a UtsNameBuf) -> Self {
		let sysname = uts.as_sysname();
		let nodename = uts.as_nodename();
		let release = uts.as_release();
		let version = uts.as_version();
		let machine = uts.as_machine();
		
		#[cfg(feature = "enable_domainname")]
		let domainname = self.as_domainname();
		
		#[cfg(feature = "enable_domainname")]
		return UtsNameSlice::new(
			sysname,
			nodename,
			release,
			version,
			machine,

			domainname
		);

		#[cfg(not(feature = "enable_domainname"))]
		return UtsNameSlice::new(
			sysname,
			nodename,
			release,
			version,
			machine,

		);
	}
}