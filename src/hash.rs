
/*!
Additional hash implementations for uname
*/


use std::hash::{Hash, Hasher};
use uts_struct::slice::UtsNameSlice;
use uts_struct::buf::UtsNameBuf;
use UtsName;


///Hashing the kernel versions (Sysname + Release + Version)
pub trait HashVersion {
	///Get a hash version for this structure
	fn hash_version<H: Hasher>(&self, state: &mut H);
}

//buf
impl HashVersion for UtsNameBuf {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}


impl<'a> HashVersion for &'a UtsNameBuf {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}


//slice
impl<'a> HashVersion for UtsNameSlice<'a> {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}

impl<'a> HashVersion for &'a UtsNameSlice<'a> {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}