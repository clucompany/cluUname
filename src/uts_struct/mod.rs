
extern crate libc;

use std::os::raw::c_char;
use std::io::Error;
use std::ffi::CString;
use std::mem;

#[cfg(feature = "enable_domainname")]
mod enable_domainname;
#[cfg(feature = "enable_domainname")]
pub use self::enable_domainname::*;


#[cfg(not(feature = "enable_domainname"))]
mod not_enable_domainname;
#[cfg(not(feature = "enable_domainname"))]
pub use self::not_enable_domainname::*;





pub type UtsNameSlice<T> = UtsNameAlwaysType<T>;
pub type UtsNameThisMachine = UtsNameAlwaysType<CString>;
pub type UtsNameUTF8 = UtsNameAlwaysType<String>;


#[cfg(feature = "enable_domainname")]
pub type UtsNameAlwaysType<T> = UtsName<T, T, T, T, T, T>;

#[cfg(not(feature = "enable_domainname"))]
pub type UtsNameAlwaysType<T> = UtsName<T, T, T, T, T>;


impl UtsNameThisMachine {
	pub fn this_machine() -> Result<Self, Error> {
		let utsname: libc::utsname = unsafe { mem::zeroed() };
		Self::update_libc(utsname)
	}
	
	pub fn update_libc(mut utsname: libc::utsname) -> Result<Self, Error> {
		match unsafe { libc::uname(&mut utsname) } {
			0 => {},
			_ => return Err(Error::last_os_error()),
		}

		Ok(utsname.into())
	}
	
	/*pub fn to_utf8_uts(self) -> Result<UtsNameUTF8, UTSUTF8Err> {
		Ok({			
			#[cfg(feature = "enable_domainname")]
			let [
				sysname,
				nodename,
				release,
				version,
				machine,
				
				domainname
			] = self.to_array();
			
			#[cfg(not(feature = "enable_domainname"))]
			let [
				sysname,
				nodename,
				release,
				version,
				machine,
			] = self.to_array();
			
			UtsNameUTF8::new(
				sysname.into_utf8()?,
				nodename.into_utf8()?,
				release.into_utf8()?,
				version.into_utf8()?,
				machine.into_utf8()?,
				
				#[cfg(feature = "enable_domainname")]
				domainname.into_utf8()?,
			)	
		})
	}*/
}


impl From< libc::utsname > for UtsNameThisMachine {
	fn from(uts: libc::utsname) -> Self {
		let sysname = Box::new(uts.sysname);
		//1 1 1 1 0 0 0 0 0 0 0 0 0

		let nodename = Box::new(uts.nodename);
		let release = Box::new(uts.release);
		let version = Box::new(uts.version);
		let machine = Box::new(uts.machine);
		
		#[cfg(feature = "enable_domainname")]
		let domainname = Box::new(uts.domainname);
		
		
		let result = unsafe {UtsNameThisMachine::new( 
			CString::from_raw(sysname.as_ptr() as *mut c_char),
			CString::from_raw(nodename.as_ptr() as *mut c_char),
			CString::from_raw(release.as_ptr() as *mut c_char),
			CString::from_raw(version.as_ptr() as *mut c_char),
			CString::from_raw(machine.as_ptr() as *mut c_char),
			
			#[cfg(feature = "enable_domainname")]
			CString::from_raw(domainname.as_ptr() as *mut c_char),
		)};

		//Why there is no leakage?
		//
		//CString becomes a destructor!!!
		//And the values of CString are not released before the time
		//
		
		mem::forget(sysname);
		mem::forget(nodename);
		mem::forget(release);
		mem::forget(version);
		mem::forget(machine);
		
		#[cfg(feature = "enable_domainname")]
		mem::forget(domainname);
		
		result
	}
}
