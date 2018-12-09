
/*!
Additional hash implementations for uname
*/

use crate::uts_struct::UtsNameSlice;
use crate::uts_struct::UtsNameBuf;
use crate::UtsName;

use std::hash::{Hash, Hasher};



///Hashing the kernel versions (Sysname + Release + Version)
pub trait HashVersion {
	///Get a hash version for this structure
	fn hash_version<H: Hasher>(&self, state: &mut H);
}

impl<'a, T: HashVersion> HashVersion for &'a T {
	#[inline(always)]
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		(*self).hash_version(state)
	}
}
impl<'a, T: HashVersion> HashVersion for &'a mut T {
	#[inline(always)]
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		(**self).hash_version(state)
	}
}



//buf
impl HashVersion for UtsNameBuf {
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
