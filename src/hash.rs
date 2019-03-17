
/*!
Additional hash implementations for uname
*/

use std::hash::Hasher;

///Hashing the kernel versions (Sysname + Release + Version)
pub trait HashVersion {
	///Get a hash version for this structure
	fn hash_version<H: Hasher>(&self, state: &mut H);
}

impl<'a, T: HashVersion> HashVersion for &'a T {
	#[inline(always)]
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		T::hash_version(self, state)
	}
}
impl<'a, T: HashVersion> HashVersion for &'a mut T {
	#[inline(always)]
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		T::hash_version(self, state)
	}
}

impl<T: HashVersion> HashVersion for Box<T> {
	#[inline(always)]
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		T::hash_version(self, state)
	}
}

