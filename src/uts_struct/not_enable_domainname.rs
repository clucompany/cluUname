use crate::uts_struct::UtsNameAlwaysType;
use crate::hash::HashVersion;
use std::hash::Hasher;
use std::fmt::Display;
use crate::display::DisplayUts;
use std::hash::Hash;
use crate::element::UtsElement;
use std::fmt;
use crate::uts_struct::UtsNameUTF8;
use crate::element::UTSUTF8Err;
use crate::element::UtsElementIntoUTF;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UtsName<Q, W, E, R, T>  where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	sysname:	Q,
	nodename:	W,
	release:	E,
	version:	R,
	machine:	T,
}

impl<Q, W, E, R, T> From<(Q, W, E, R, T)> for UtsName<Q, W, E, R, T> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	#[inline(always)]
	fn from((q, w, e, r, t): (Q, W, E, R, T)) -> Self {
		Self::new(q, w, e, r, t)	
	}
}


impl<'a, Q:'a, W:'a, E:'a, R:'a, T:'a> UtsName<Q, W, E, R, T> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement{	
	#[inline(always)]
	pub const fn as_sysname(&self) -> &(dyn UtsElement + 'a) {
		&self.sysname
	}
	
	#[inline(always)]
	pub const fn as_nodename(&self) -> &(dyn UtsElement + 'a) {
		&self.nodename
	}
	
	#[inline(always)]
	pub const fn as_release(&self) -> &(dyn UtsElement + 'a) {
		&self.release
	}
	
	#[inline(always)]
	pub const fn as_version(&self) -> &(dyn UtsElement + 'a) {
		&self.version
	}
	
	#[inline(always)]
	pub const fn as_machine(&self) -> &(dyn UtsElement + 'a) {
		&self.machine
	}
	
	
	
	pub fn uname_hash(&self) -> u64 where Q: Hash, W: Hash, E: Hash, R: Hash, T: Hash {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);

		hasher.finish()  
	}
	
	pub fn version_hash(&self) -> u64 where Q: Hash, E: Hash, R: Hash {		
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash_version(&mut hasher);
		
		hasher.finish()  
	}
	
	#[inline]
	///Display trait for sysname.
	pub const fn display_sysname<'r>(&'r self) -> DisplayUts<&Q> {
		DisplayUts::new(&self.sysname)
	}
	
	///Display trait for nodename.
	#[inline]
	pub const fn display_nodename<'r>(&'r self) -> DisplayUts<&W> {
		DisplayUts::new(&self.nodename)
	}
	
	///Display trait for release.
	#[inline]
	pub const fn display_release<'r>(&'r self) -> DisplayUts<&E> {
		DisplayUts::new(&self.release)
	}
	
	///Display trait for version.
	#[inline]
	pub const fn display_version<'r>(&'r self) -> DisplayUts<&R> {
		DisplayUts::new(&self.version)
	}
	
	///Display trait for machine.
	#[inline]
	pub const fn display_machine<'r>(&'r self) -> DisplayUts<&T> {
		DisplayUts::new(&self.machine)
	}
	
	#[inline]
	pub const fn new(q:Q, w:W, e:E, r:R, t:T) -> Self {
		UtsName {
			sysname:	q,
			nodename:	w,
			release:	e,
			version:	r,
			machine:	t,
		}
	}
	
	pub fn as_bytes(&self) -> [&[u8]; 5] {
		[
			self.sysname.as_array(),
			self.nodename.as_array(),
			self.release.as_array(),
			self.version.as_array(),
			self.machine.as_array(),
		]
	}
	#[inline(always)]
	pub fn as_bytes0(&self) -> [&[u8]; 5] {
		self.as_bytes()
	}
	
	
	pub fn to_utf8(self) -> Result<UtsNameUTF8, UTSUTF8Err> where Q: UtsElementIntoUTF, W: UtsElementIntoUTF, E: UtsElementIntoUTF, R: UtsElementIntoUTF, T: UtsElementIntoUTF {
		Ok({			
			UtsNameUTF8::new(
				self.sysname.into_utf8()?,
				self.nodename.into_utf8()?,
				self.release.into_utf8()?,
				self.version.into_utf8()?,
				self.machine.into_utf8()?,
			)	
		})
	}
}


impl<Q, W, E, R, T> Display for UtsName<Q, W, E, R, T> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, 
			"{} {} {} {} {}",
			
			self.as_sysname(),
			self.as_nodename(),
			self.as_release(),
			self.as_version(),
			self.as_machine(),
		)
	}
}

impl<Q, W, E, R, T> HashVersion for UtsName<Q, W, E, R, T> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	fn hash_version<H: Hasher>(&self, state: &mut H) {
		self.sysname.as_array().hash(state);
		self.release.as_array().hash(state);
		self.version.as_array().hash(state);
		
		//HASH CSTR != HASH STR
	}
}

impl<Q, W, E, R, T> Hash for UtsName<Q, W, E, R, T> where Q: UtsElement, W: UtsElement, E: UtsElement, R: UtsElement, T: UtsElement {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.sysname.as_array().hash(state);
		self.nodename.as_array().hash(state);
		self.release.as_array().hash(state);
		self.version.as_array().hash(state);
		self.machine.as_array().hash(state);
		
		//HASH CSTR != HASH STR
	}
}


impl<T> UtsNameAlwaysType<T> where T: UtsElement {
	#[inline]
	pub const fn as_array(&self) -> [&T; 5] {
		[
			&self.sysname,
			&self.nodename,
			&self.release,
			&self.version,
			&self.machine,
		]
	}
	
	#[inline(always)]
	pub const fn as_array0(&self) -> [&T; 5] {
		self.as_array()
	}
	

	#[inline]
	pub fn to_array(self) -> [T; 5] {
		[
			self.sysname,
			self.nodename,
			self.release,
			self.version,
			self.machine,
		]
	}
	
	#[inline(always)]
	pub fn to_array0(self) -> [T; 5] {
		self.to_array()	
	}
}