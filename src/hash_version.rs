
/*!
Hash trait for kernel version.
*/


use std::hash::{Hash, Hasher};
use uts_struct::slice::UtsNameSlice;
use uts_struct::buf::UtsNameBuf;
use UtsName;


pub trait HashVersion {
	///Get a hash version for this structure
	fn hash_version<H: Hasher>(&self, state: &mut H);
}

impl HashVersion for UtsNameBuf {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}

impl<'a> HashVersion for UtsNameSlice<'a> {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.as_sysname().hash(state);
		self.as_release().hash(state);
		self.as_version().hash(state);
	}
}
